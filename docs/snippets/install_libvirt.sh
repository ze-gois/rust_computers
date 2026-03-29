#!/usr/bin/env bash
#
# install_libvirt.sh — snippet to install and enable libvirt/qemu/ovmf/virt-manager
#
# Purpose:
#  - Provide a safe, documented script for installing the core virtualization stack
#    used in the `computers` project (KVM/QEMU + libvirt + OVMF + virt-manager).
#  - Works on Arch Linux and Debian/Ubuntu derivatives (detects distro via /etc/os-release).
#  - Includes checks, suggested post-steps, and non-destructive defaults.
#
# Usage:
#  - Review this file before executing.
#  - Dry-run (print actions):    ./install_libvirt.sh --dry-run
#  - Interactive (default):      ./install_libvirt.sh
#  - Non-interactive (assume yes): ./install_libvirt.sh --yes
#  - Force distro:               ./install_libvirt.sh --distro arch|debian
#
# CAUTION:
#  - This script runs package manager commands and systemctl enable/start.
#  - Always inspect and understand changes before running on a production machine.
#  - Keep a live USB / recovery plan when changing core system services.
#

set -euo pipefail

# Default behavior
DRY_RUN=0
ASSUME_YES=0
FORCE_DISTRO=""

# Helpers
info()  { printf '\033[1;34m[INFO]\033[0m %s\n' "$*"; }
warn()  { printf '\033[1;33m[WARN]\033[0m %s\n' "$*"; }
error() { printf '\033[1;31m[ERROR]\033[0m %s\n' "$*"; exit 1; }

usage() {
  cat <<EOF
install_libvirt.sh — install libvirt/QEMU/OVMF/virt-manager (Arch + Debian/Ubuntu)

Options:
  --dry-run         : print actions without executing them
  --yes             : assume yes for package manager prompts / non-interactive
  --distro <name>   : force distro detection (arch or debian)
  -h, --help        : show this help

Example:
  # interactive:
  ./install_libvirt.sh

  # non-interactive:
  ./install_libvirt.sh --yes

EOF
  exit 0
}

parse_args() {
  while [ $# -gt 0 ]; do
    case "$1" in
      --dry-run) DRY_RUN=1; shift ;;
      --yes) ASSUME_YES=1; shift ;;
      --distro) FORCE_DISTRO="$2"; shift 2 ;;
      -h|--help) usage ;;
      *) echo "Unknown arg: $1"; usage ;;
    esac
  done
}

run_cmd() {
  if [ "$DRY_RUN" -eq 1 ]; then
    printf '[DRY-RUN] %s\n' "$*"
  else
    if [ "$ASSUME_YES" -eq 1 ]; then
      # Run command non-interactively where applicable
      eval "$*"
    else
      eval "$*"
    fi
  fi
}

detect_distro() {
  if [ -n "$FORCE_DISTRO" ]; then
    echo "$FORCE_DISTRO"
    return
  fi

  if [ -f /etc/os-release ]; then
    . /etc/os-release
    case "${ID,,}" in
      arch|manjaro) echo "arch" ;;
      ubuntu|debian|pop|kali) echo "debian" ;;
      *)
        # Try ID_LIKE
        if [ -n "${ID_LIKE-}" ]; then
          case "${ID_LIKE,,}" in
            *arch*) echo "arch" ;;
            *debian*|*ubuntu*) echo "debian" ;;
            *) echo "unknown" ;;
          esac
        else
          echo "unknown"
        fi
        ;;
    esac
  else
    echo "unknown"
  fi
}

preflight_checks() {
  info "Running preflight checks"
  # Must run as a user with sudo available
  if ! command -v sudo >/dev/null 2>&1; then
    warn "sudo not found — script expects sudo to be available for privileged operations."
  fi

  if [ "$(id -u)" -eq 0 ]; then
    warn "Running as root. The script will still use 'sudo' where appropriate. Proceeding as root."
  fi

  if [ "$DRY_RUN" -eq 1 ]; then
    info "Dry-run enabled: no changes will be made."
  fi
}

install_on_arch() {
  info "Preparing to install on Arch/Arch-derivative"

  PKGS="qemu libvirt edk2-ovmf virt-manager dnsmasq"
  # Optional helpful packages
  OPTIONAL="bridge-utils openbsd-netcat"

  info "Packages to install: $PKGS"
  info "Optional packages: $OPTIONAL"

  if [ "$DRY_RUN" -eq 1 ]; then
    run_cmd "sudo pacman -Syu --noconfirm $PKGS $OPTIONAL"
  else
    if [ "$ASSUME_YES" -eq 1 ]; then
      run_cmd "sudo pacman -Syu --noconfirm $PKGS $OPTIONAL"
    else
      echo
      printf 'About to run: sudo pacman -Syu %s %s\n' "$PKGS" "$OPTIONAL"
      read -r -p "Proceed with installation? [y/N] " ans
      case "$ans" in
        y|Y) run_cmd "sudo pacman -Syu $PKGS $OPTIONAL" ;;
        *) info "Installation aborted by user." ; exit 0 ;;
      esac
    fi
  fi

  # enable and start libvirtd
  run_cmd "sudo systemctl enable --now libvirtd"
}

install_on_debian() {
  info "Preparing to install on Debian/Ubuntu"

  PKGS="qemu-kvm libvirt-daemon-system libvirt-clients virt-manager ovmf dnsmasq"
  # On some distros libvirt-daemon-system pulls libvirt and related dependencies
  info "Packages to install: $PKGS"

  if [ "$DRY_RUN" -eq 1 ]; then
    run_cmd "sudo apt update && sudo apt install -y $PKGS"
  else
    if [ "$ASSUME_YES" -eq 1 ]; then
      run_cmd "sudo apt update && sudo apt install -y $PKGS"
    else
      echo
      printf 'About to run: sudo apt update && sudo apt install -y %s\n' "$PKGS"
      read -r -p "Proceed with installation? [y/N] " ans
      case "$ans" in
        y|Y) run_cmd "sudo apt update && sudo apt install -y $PKGS" ;;
        *) info "Installation aborted by user." ; exit 0 ;;
      esac
    fi
  fi

  # enable and start libvirtd (service name libvirtd or libvirtd?)
  if systemctl list-unit-files | grep -q '^libvirtd'; then
    run_cmd "sudo systemctl enable --now libvirtd"
  else
    # some Ubuntu flavors use libvirt-bin or libvirt-daemon
    run_cmd "sudo systemctl enable --now libvirtd || sudo systemctl enable --now libvirt-bin || true"
  fi
}

post_install_tips() {
  cat <<EOF

Post-installation tips (please read):

1) Add your user to the libvirt group so you can manage VMs without root:
   sudo usermod -aG libvirt $(whoami)
   # then log out/in or run: newgrp libvirt

2) Confirm libvirt is running:
   systemctl status libvirtd

3) Test QEMU/KVM availability:
   # shows KVM modules and CPU accel
   sudo virsh -c qemu:///system list --all

4) OVMF (UEFI) firmware:
   - Arch: package 'edk2-ovmf' contains OVMF firmware for virt-manager/OVMF guests.
   - Debian/Ubuntu: package 'ovmf' provides the firmware.

5) If you plan GPU passthrough:
   - Ensure the host has IOMMU enabled (kernel cmdline: intel_iommu=on iommu=pt or amd_iommu=on)
   - Add vfio modules to your initramfs (Arch: /etc/mkinitcpio.conf MODULES=(vfio_pci vfio vfio_iommu_type1))
   - See the project docs/snippets for detailed steps and safety checks.

6) If you use a desktop: you may prefer to run 'virt-manager' as an unprivileged user (after adding to group libvirt).

7) Firewall / networking:
   - By default libvirt may create a NAT network. If you want VMs on the same LAN, configure bridge networking.
   - For simple host <-> VM TCP exposure, SSH tunnel is a safe starting point.

EOF
}

main() {
  parse_args "$@"
  preflight_checks

  DISTRO=$(detect_distro)
  info "Detected distro: $DISTRO"

  case "$DISTRO" in
    arch)
      install_on_arch
      ;;
    debian)
      install_on_debian
      ;;
    unknown)
      warn "Distribution could not be detected automatically."
      echo "You can re-run this script with --distro arch or --distro debian."
      exit 1
      ;;
    *)
      warn "Unsupported distro: $DISTRO"
      exit 1
      ;;
  esac

  post_install_tips

  info "Done. Remember to logout/login for group changes to take effect."
}

main "$@"
