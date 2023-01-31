#!/bin/bash
set -u

die() {
    echo "$@" 1>&2
    exit 1
}

mount_path="/mnt"
pico_part="$(lsblk -o label,path | egrep '^RPI-RP2' | awk '{print $2}')"
[ -z "$pico_part" ] && die "Could not find a connected pico device"

mount | grep -Fq " on $mount_path " && die "$mount_path is already mounted, stopping for safety"
mount | grep -Eq "^$pico_part" && die "$pico_part is already mounted, stopping for safety"

echo "Mounting $pico_part to $mount_path"
sudo mount "$pico_part" "$mount_path" -o "uid=$(id -u),gid=$(id -g)"
cargo run --release
sudo umount "$pico_part"
