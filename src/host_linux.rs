//! Linux host hardware observations backed by sysfs and procfs.
//!
//! Every value is read from kernel-exposed interfaces, with no shell commands
//! and no external dependencies. Unreadable or absent values remain absent.

use std::fs;
use std::path::Path;

use crate::hardware::{Device, Inventory, Property};

/// An owned snapshot that can be rendered after collection.
#[derive(Debug, Default)]
pub struct Snapshot {
    devices: Vec<OwnedDevice>,
}

#[derive(Debug)]
struct OwnedDevice {
    category: &'static str,
    name: String,
    properties: Vec<OwnedProperty>,
}

#[derive(Debug)]
struct OwnedProperty {
    name: &'static str,
    value: String,
}

impl Snapshot {
    /// Collects hardware made visible by the running Linux kernel.
    ///
    /// Missing interfaces are skipped rather than fabricated. The result is a
    /// snapshot, not a measurement of real-time resource usage.
    pub fn collect() -> std::io::Result<Self> {
        collect_from(Path::new("/sys"), Path::new("/proc"))
    }

    /// Render this snapshot into any `core::fmt::Write` sink.
    pub fn write_log(&self, output: &mut impl core::fmt::Write) -> core::fmt::Result {
        for device in &self.devices {
            let properties: Vec<_> = device.properties.iter().map(|property| Property {
                name: property.name,
                value: property.value.as_str(),
            }).collect();
            let device = [Device {
                category: device.category,
                name: device.name.as_str(),
                properties: &properties,
            }];
            Inventory { devices: &device }.write_log(output)?;
        }
        Ok(())
    }
}

fn collect_from(sys: &Path, proc: &Path) -> std::io::Result<Snapshot> {
    let mut snapshot = Snapshot::default();
    let cpuinfo = proc.join("cpuinfo");
    if let Ok(info) = fs::read_to_string(cpuinfo) {
        let model = info.lines().find_map(|line| field(line, "model name"))
            .or_else(|| info.lines().find_map(|line| field(line, "Hardware")));
        let logical = info.lines().filter(|line| field(line, "processor").is_some()).count();
        if model.is_some() || logical != 0 {
            let mut properties = Vec::new();
            if logical != 0 {
                properties.push(OwnedProperty { name: "logical_processors", value: logical.to_string() });
            }
            snapshot.devices.push(OwnedDevice {
                category: "processor",
                name: model.unwrap_or("unknown").to_owned(),
                properties,
            });
        }
    }

    if let Some(memory) = read_memtotal(&proc.join("meminfo")) {
        snapshot.devices.push(OwnedDevice {
            category: "memory",
            name: "system".into(),
            properties: vec![OwnedProperty { name: "total_kib", value: memory }],
        });
    }

    collect_directory(&mut snapshot, &sys.join("bus/pci/devices"), "pci", &[
        ("vendor", "vendor"), ("device", "device"), ("class", "class"),
        ("subsystem_vendor", "subsystem_vendor"), ("subsystem_device", "subsystem_device"),
    ])?;
    collect_directory(&mut snapshot, &sys.join("block"), "block", &[
        ("size", "sectors_512_bytes"),
        ("device/model", "model"),
        ("device/vendor", "vendor"),
    ])?;
    collect_directory(&mut snapshot, &sys.join("class/power_supply"), "power_supply", &[
        ("type", "type"), ("manufacturer", "manufacturer"),
        ("model_name", "model"), ("energy_full", "energy_full_microwatt_hours"),
        ("energy_full_design", "energy_full_design_microwatt_hours"),
        ("capacity", "capacity_percent"),
    ])?;
    Ok(snapshot)
}

fn field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let (key, value) = line.split_once(':')?;
    (key.trim() == name).then(|| value.trim())
}

fn read_memtotal(path: &Path) -> Option<String> {
    let data = fs::read_to_string(path).ok()?;
    let value = data.lines().find_map(|line| field(line, "MemTotal"))?;
    let number = value.strip_suffix(" kB")?.trim();
    number.parse::<u64>().ok().map(|number| number.to_string())
}

fn read_property(path: &Path) -> Option<String> {
    let value = fs::read_to_string(path).ok()?;
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

fn collect_directory(
    snapshot: &mut Snapshot,
    directory: &Path,
    category: &'static str,
    fields: &[(&str, &'static str)],
) -> std::io::Result<()> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound
            || error.kind() == std::io::ErrorKind::PermissionDenied => return Ok(()),
        Err(error) => return Err(error),
    };
    let mut paths = entries.filter_map(Result::ok).map(|entry| entry.path()).collect::<Vec<_>>();
    paths.sort();
    for path in paths {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else { continue };
        let properties = fields.iter().filter_map(|(file, name)| {
            read_property(&path.join(file)).map(|value| OwnedProperty { name, value })
        }).collect();
        snapshot.devices.push(OwnedDevice {
            category,
            name: name.to_owned(),
            properties,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_exact_fields() {
        assert_eq!(field("model name : Example CPU", "model name"), Some("Example CPU"));
        assert_eq!(field("model : Other", "model name"), None);
    }

    #[test]
    fn missing_files_are_not_fake_devices() {
        let directory = std::env::temp_dir().join(format!("computers-nonexistent-{}", std::process::id()));
        let inventory = collect_from(&directory.join("sys"), &directory.join("proc")).unwrap();
        assert!(inventory.devices.is_empty());
    }
}
