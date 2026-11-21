#![no_std]

//! Descriptions of computers and their hardware, independent of any logger.

pub mod hardware;

#[cfg(all(feature = "host-linux", target_os = "linux"))]
extern crate std;

#[cfg(all(feature = "host-linux", target_os = "linux"))]
pub mod host_linux;

/// Retained for compatibility with the original scaffold.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
