//! Run on Linux with:
//! cargo run --example host_hardware --features host-linux --target x86_64-unknown-linux-gnu

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let snapshot = computers::host_linux::Snapshot::collect()?;
    let mut report = String::new();
    snapshot.write_log(&mut report)?;
    print!("{report}");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("The host-linux inventory example requires Linux");
}
