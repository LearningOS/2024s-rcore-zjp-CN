//! File trait & inode(dir, file, pipe, stdin, stdout)

mod inode;
mod pipe;
mod stdio;

use crate::mm::UserBuffer;

/// trait File for all file types
pub trait File: Send + Sync {
    /// the file readable?
    fn readable(&self) -> bool;
    /// the file writable?
    fn writable(&self) -> bool;
    /// read from the file to buf, return the number of bytes read
    fn read(&self, buf: UserBuffer) -> usize;
    /// write to the file from buf, return the number of bytes written
    fn write(&self, buf: UserBuffer) -> usize;
    /// stats
    fn stats(&self) -> Stat {
        Stat {
            dev: 0,
            ino: 0,
            mode: StatMode::NULL,
            nlink: 1,
            pad: [0; 7],
        }
    }
}

/// The stat of a inode
#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    /// ID of device containing file
    pub dev: u64,
    /// inode number
    pub ino: u64,
    /// file type and mode
    pub mode: StatMode,
    /// number of hard links
    pub nlink: u32,
    /// unused pad
    pad: [u64; 7],
}

bitflags! {
    /// The mode of a inode
    /// whether a directory or a file
    pub struct StatMode: u32 {
        /// null
        const NULL  = 0;
        /// directory
        const DIR   = 0o040000;
        /// ordinary regular file
        const FILE  = 0o100000;
    }
}

/// hard link
pub fn linkat(old: &str, new: &str) -> Option<Arc<Inode>> {
    let old_inode_id = ROOT_INODE.get_inode_id(old).unwrap();
    ROOT_INODE.linkat(new, old_inode_id)
}

/// remove a hard link
pub fn unlinkat(name: &str) -> bool {
    ROOT_INODE.unlinkat(name)
}

use self::inode::ROOT_INODE;
use alloc::sync::Arc;
use easy_fs::Inode;
pub use inode::{list_apps, open_file, OSInode, OpenFlags};
pub use pipe::{make_pipe, Pipe};
pub use stdio::{Stdin, Stdout};
