# `ipod-read-sysinfo-extended-macos`

A small Rust program to read the extended SysInfo data from newer models of iPods on macOS.

## Installation

You can download a command line binary from the [releases](https://github.com/dstaley/ipod-read-sysinfo-extended-macos/releases/latest) page.

## Compatibility

This utility only supports iPods that expose their extended SysInfo via USB Control Transfers.

- iPod Classic (6th generation)
- iPod nano (3rd, 4th, 5th generations)
- iPod shuffle (3rd, 4th generations)

## Usage

First, determine the path to your iPod's `Device` folder by running `find /Volumes -type d -name 'Device' -d 3`.

```
$ find /Volumes -type d -name 'Device' -d 3
/Volumes/IPOD/iPod_Control/Device
find: /Volumes/IPOD/.Trashes: Operation not permitted
```

Then, run `ipod-read-sysinfo-extended-macos` and pipe the output to a file named `SysInfoExtended` in your iPod's `Device` folder.

```
$ ./ipod-read-sysinfo-extended-macos > /Volumes/IPOD/iPod_Control/Device/SysInfoExtended
```