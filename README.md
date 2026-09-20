# computers

[![crates.io](https://img.shields.io/crates/v/computers.svg)](https://crates.io/crates/computers)
[![docs.rs](https://docs.rs/computers/badge.svg)](https://docs.rs/computers)

Computer, virtualization, emulation, and machine-level experimentation for the [userspace.party](https://userspace.party) ecosystem.

## Role

`computers` is the family member for representing and experimenting with computers as systems: architecture, virtualization, emulation, hypervisor-facing work, and the execution environments built above them.

It sits above `ample` and `userspace`, and uses `userspace_build` for build-side integration.

## Current state

The Rust crate is currently a small `no_std` scaffold with a freestanding binary/startup path. The repository also contains historical notes under `docs/` from virtualization, VFIO, QEMU/KVM and GPU-passthrough experiments.

Those notes are useful research material, but they are not presented as the stable crate API. As reusable machine abstractions emerge, they should move into the Rust surface and be documented through rustdoc.

## Use

```bash
cargo add computers
```

At this stage, consumers should expect the public API to change substantially.

## Development direction

The crate's scope includes areas such as:

- machine and architecture description;
- virtualization and hypervisor boundaries;
- emulation;
- guest/host execution environments;
- hardware resources exposed to higher layers of the ecosystem.

Concrete APIs are added only as the corresponding implementation becomes real.

## Ecosystem

- Ecosystem: https://userspace.party
- Crate homepage: https://userspace.party/computers
- API documentation: https://docs.rs/computers
- crates.io: https://crates.io/crates/computers
- Source: https://github.com/ze-gois/rust_computers
- Workspace hub: https://github.com/ze-gois/rust_userspace_hub

## Status

Early experimental systems crate. The repository contains active research context, but the reusable Rust API remains intentionally small.

## License

See [LICENSE](LICENSE).
