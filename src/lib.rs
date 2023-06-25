//! # FdLock
//!
//! `FdLock` is a Rust crate that provides functionality for file locking using `flock` or `fcntl` operations.
//!
//! This crate defines a trait `FdLock` that extends the `AsRawFd` trait,
//! allowing file locks to be placed on file descriptors. It supports both
//! shared and exclusive locks, as well as unlocking operations.
//!
//! ## Examples
//!
//! Placing a shared lock on a file:
//!
//! ```no_run
//! use filelock_rs::FdLock;
//! use std::fs::File;
//!
//! let file = File::open("data.txt").expect("Failed to open file");
//! let lock_result = file.lock_shared();
//!
//! match lock_result {
//!     Ok(()) => {
//!         println!("Shared lock placed on the file");
//!         // Perform operations with the locked file
//!     }
//!     Err(error) => {
//!         eprintln!("Failed to place a shared lock: {}", error);
//!     }
//! }
//! ```
//!
//! Placing an exclusive lock on a file:
//!
//! ```rust
//! use filelock_rs::FdLock;
//! use std::fs::File;
//!
//! let file = File::create("data.txt").expect("Failed to create file");
//! let lock_result = file.lock_exclusive();
//!
//! match lock_result {
//!     Ok(()) => {
//!         println!("Exclusive lock placed on the file");
//!         // Perform operations with the locked file
//!     }
//!     Err(error) => {
//!         eprintln!("Failed to place an exclusive lock: {}", error);
//!     }
//! }
//! ```
//!
//! ## Cleanup
//!
//! The `FdLock` trait is implemented for the `std::fs::File` type. When the `FdLock` methods are
//! used on a `File` instance, the locks are automatically released when they go out of scope.
//!
//! ## Notes
//!
//! - The behavior of file locking may differ depending on the operating system.
//! - The crate uses the `libc` and `io::Result` types from the standard library.
//! - If the file lock operation fails, an `io::Error` is returned.
pub mod pid;

use std::io;
use std::os::fd::AsRawFd;

/// FdLock Operation type.
pub type Operation = libc::c_int;

/// Place a shared lock. More than one process may hold a shared lock for a given file at a given time.
#[allow(dead_code)]
const LOCK_SH: Operation = libc::LOCK_SH;
/// Place an exclusive lock. Only one process may hold an exclusive lock for a given file at a given time.
#[allow(dead_code)]
const LOCK_EX: Operation = libc::LOCK_EX;
/// Remove an existing lock held by this process.
#[allow(dead_code)]
const LOCK_UN: Operation = libc::LOCK_UN;

/// The `FdLock` trait extends the `AsRawFd` trait, allowing
/// file locks to be placed on file descriptors.
pub trait FdLock: AsRawFd {
    /// Places a file lock on the associated file descriptor using the `flock` operation.
    ///
    /// # Arguments
    ///
    /// * `operation`: The type of lock to place on the file.
    ///
    /// # Errors
    ///
    /// If the lock operation fails, an `io::Error` is returned.
    ///
    #[cfg(not(target_os = "solaris"))]
    fn flock(&self, operation: Operation) -> io::Result<()> {
        let ret = unsafe { libc::flock(self.as_raw_fd(), operation) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Places a file lock on the associated file descriptor using the `flock` operation.
    ///
    /// # Arguments
    ///
    /// * `operation`: The type of lock to place on the file.
    ///
    /// # Errors
    ///
    /// If the lock operation fails, an `io::Error` is returned.
    ///
    #[cfg(target_os = "solaris")]
    fn flock(&self, operation: Operation) -> io::Result<()> {
        // Solaris lacks flock(), so try to emulate using fcntl()
        let mut flock = libc::flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_sysid: 0,
            l_pid: 0,
            l_pad: [0, 0, 0, 0],
        };
        flock.l_type = if operation & LOCK_UN != 0 {
            LOCK_UN
        } else if operation & LOCK_EX != 0 {
            libc::F_WRLCK
        } else if operation & LOCK_SH != 0 {
            libc::F_RDLCK
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("unexpected flock() operation"),
            ));
        };

        let mut cmd = libc::F_SETLKW;
        if (flag & libc::LOCK_NB) != 0 {
            cmd = libc::F_SETLK;
        }

        let ret = unsafe { libc::fcntl(file.as_raw_fd(), cmd, &flock) };
        if ret < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    /// Places a shared lock on the file.
    ///
    /// This method uses the `LOCK_SH` operation to place a shared lock on the associated file descriptor.
    ///
    /// # Errors
    ///
    /// If the lock operation fails, an `io::Error` is returned.
    ///
    fn lock_shared(&self) -> io::Result<()> {
        self.flock(libc::LOCK_SH)
    }

    /// Places an exclusive lock on the file.
    ///
    /// This method uses the `LOCK_EX` operation to place an exclusive lock on the associated file descriptor.
    ///
    /// # Errors
    ///
    /// If the lock operation fails, an `io::Error` is returned.
    ///
    fn lock_exclusive(&self) -> io::Result<()> {
        self.flock(libc::LOCK_EX)
    }

    /// Tries to place a shared lock on the file.
    ///
    /// This method uses the `LOCK_SH | LOCK_NB` operations to try placing a shared lock on the associated file descriptor.
    ///
    /// # Errors
    ///
    /// If the lock operation fails or the lock is not immediately available, an `io::Error` is returned.
    ///
    fn try_lock_shared(&self) -> io::Result<()> {
        self.flock(libc::LOCK_SH | libc::LOCK_NB)
    }

    /// Tries to place an exclusive lock on the file.
    ///
    /// This method uses the `LOCK_EX | LOCK_NB` operations to try placing an exclusive lock on the associated file descriptor.
    ///
    /// # Errors
    ///
    /// If the lock operation fails or the lock is not immediately available, an `io::Error` is returned.
    ///
    fn try_lock_exclusive(&self) -> io::Result<()> {
        self.flock(libc::LOCK_EX | libc::LOCK_NB)
    }

    /// Unlocks the file.
    ///
    /// This method removes the lock held by the current process on the associated file descriptor.
    /// It uses the `LOCK_UN` operation to unlock the file.
    ///
    /// # Errors
    ///
    /// If the unlock operation fails, an `io::Error` is returned.
    ///
    fn unlock(&self) -> io::Result<()> {
        self.flock(libc::LOCK_UN)
    }
}

impl FdLock for std::fs::File {}
