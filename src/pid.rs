//! # Pid
//!
//! `Pid` is a Rust crate that provides functionality for creating, reading, and managing PID (Process ID) files.
//!
//! A PID file typically contains the process ID of a running program. This crate
//! offers an easy-to-use interface for working with PID files, allowing you to
//! create new PID files, read process IDs from existing files, and handle the
//! cleanup of PID files when they are no longer needed.
//!
//! ## Examples
//!
//! Creating a new PID file:
//!
//! ```no_run
//! use filelock_rs::pid::Pid;
//!
//! match Pid::new("/var/run", "my_program") {
//!   Ok(pid) => {
//!      println!("PID file created. Process ID: {}", pid.process_id);
//!   }
//!   Err(error) => {
//!      eprintln!("Failed to create PID file: {}", error);
//!   }
//! }
//! ```
//!
//! Reading the process ID from an existing PID file:
//!
//! ```no_run
//! use filelock_rs::pid::Pid;
//!
//! let pid_file = Pid::new("/var/run", "my_program").expect("Failed to create PID file");
//! let process_id = pid_file.process_id;
//! println!("Process ID: {}", process_id);
//! ```
//!
//! ## Cleanup
//!
//! The `Pid` structure implements the `Drop` trait, ensuring that the PID file
//! is automatically cleaned up when it goes out of scope. The file
//! is unlocked and removed from the file system.
//!
//! ## Notes
//!
//! - The `Pid` crate uses the standard library's file I/O and process ID functionality.
//! - Ensure that the target directory has proper write permissions for creating and manipulating PID files.
//! - If the PID file cannot be opened, locked, or written, an `std::io::Error` will be returned.
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Write;

use crate::FdLock;

/// Represents a PID (Process ID) file.
///
/// The `Pid` structure provides functionality
/// for creating, reading, and managing a PID file,
/// which typically contains the process ID of
/// a running program.
pub struct Pid {
    /// process_id stored inside the file.
    pub process_id: u32,
    /// path of the file.
    pub file_path: String,
    file: File,
}

impl Pid {
    /// Creates a new `Pid` instance with the specified file path and name.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the PID file will be stored.
    /// * `name` - The name of the PID file (without the extension).
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if the file cannot be opened, locked, or written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use filelock_rs::pid::Pid;
    ///
    /// let pid = Pid::new("/var/run", "my_program").unwrap();
    /// ```
    pub fn new<T: Display>(path: T, name: T) -> io::Result<Self> {
        let pid = std::process::id();
        let file_path = format!("{path}/{name}.pid");
        let mut file = std::fs::File::create(file_path.clone())?;
        file.try_lock_exclusive()?;
        file.write_all(format!("{pid}").as_bytes())?;
        Ok(Self {
            process_id: pid,
            file_path,
            file,
        })
    }
}

impl Drop for Pid {
    fn drop(&mut self) {
        self.file.unlock().unwrap();
        std::fs::remove_file(self.file_path.clone()).unwrap();
    }
}
