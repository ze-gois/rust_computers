//! Hardware observations and a portable text report.
//!
//! Values describe what the host operating system exposes at collection time,
//! rather than asserting that every physical component has been discovered.

use core::fmt::{self, Write};

/// One property observed from a host hardware interface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Property<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

/// A component identified by the host, with zero or more properties.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Device<'a> {
    pub category: &'a str,
    pub name: &'a str,
    pub properties: &'a [Property<'a>],
}

/// A borrowed view of hardware exposed by a host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Inventory<'a> {
    pub devices: &'a [Device<'a>],
}

impl Inventory<'_> {
    /// Writes a deterministic, line-oriented report without allocating.
    ///
    /// This is a formatting sink: it works with log adapters, serial consoles,
    /// string buffers and other implementations of `core::fmt::Write`.
    pub fn write_log(&self, output: &mut impl Write) -> fmt::Result {
        for device in self.devices {
            writeln!(output, "hardware.{}: {}", device.category, device.name)?;
            for property in device.properties {
                writeln!(
                    output,
                    "  {}: {}",
                    property.name,
                    property.value
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_devices_and_properties() {
        let properties = [Property {
            name: "vendor",
            value: "example",
        }];
        let devices = [Device {
            category: "processor",
            name: "sample",
            properties: &properties,
        }];
        let mut output = String::new();
        Inventory { devices: &devices }.write_log(&mut output).unwrap();
        assert_eq!(output, "hardware.processor: sample\n  vendor: example\n");
    }

    #[test]
    fn empty_inventory_writes_nothing() {
        let mut output = String::new();
        Inventory { devices: &[] }.write_log(&mut output).unwrap();
        assert!(output.is_empty());
    }
}
