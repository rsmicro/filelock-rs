# filelock-rs

[![Crates.io](https://img.shields.io/crates/v/filelock-rs.svg)](https://crates.io/crates/filelock-rs)
[![Documentation](https://docs.rs/filelock-rs/badge.svg)](https://docs.rs/filelock-rs)

filelock-rs is a Rust library that provides functionality for managing PID files and file locks.

## Features

- **Pid:** The `pid` module provides functionality for creating, reading, and managing PID files.
- **FdLock:** The `fdlock` module provides a trait `FdLock` that extends the `AsRawFd` trait, allowing file locks to be placed on file descriptors.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
filelock-rs = "0.1.0"
```

## Usage

``` rust
use filelock-rs_rs::pid::Pid;
use filelock-rs_rs::fdlock::FdLock;

fn main() {
    // Store the file in the struct of the process
    // because the pid stop to exist when this Pid instance
    // is dropped.
    let pid_file = Pid::new("/var/run", "my_app").expect("Failed to create PID file");

    let file = std::fs::File::new("/tmp/file.txt").unwrap();
    // Lock the file exclusively
    file.lock_exclusive().expect("Failed to lock PID file");

    // Perform some operations...

    // Unlock the file
    file.unlock().expect("Failed to unlock PID file");
}
```
## License

<div align="center">
  <img src="https://opensource.org/files/osi_keyhole_300X300_90ppi_0.png" width="150" height="150"/>
</div>

```
Copyright 2023 Vincenzo Palazzo <vincenzopalazzodev@gmail.com>. See LICENSE file.
```
