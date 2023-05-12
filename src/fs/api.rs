// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>

extern crate alloc;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::error::SvsmError;
use crate::string::FixedString;

/// Maximum supported length for a single filename
const MAX_FILENAME_LENGTH: usize = 64;
pub type FileName = FixedString<MAX_FILENAME_LENGTH>;

#[derive(Copy, Clone, Debug)]
pub enum FsError {
    Inval,
    FileExists,
    FileNotFound,
}

macro_rules! impl_fs_err {
    ($name:ident, $v:ident) => {
        pub fn $name() -> Self {
            Self::$v
        }
    };
}

impl FsError {
    impl_fs_err!(inval, Inval);
    impl_fs_err!(file_exists, FileExists);
    impl_fs_err!(file_not_found, FileNotFound);
}

pub trait File {
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<usize, SvsmError>;
    fn write(&self, buf: &[u8], offset: usize) -> Result<usize, SvsmError>;
    fn truncate(&self, size: usize) -> Result<usize, SvsmError>;
    fn size(&self) -> usize;
}

pub trait Directory {
    fn list(&self) -> Vec<FileName>;
    fn lookup_entry(&self, name: FileName) -> Result<DirEntry, SvsmError>;
    fn create_file(&self, name: FileName) -> Result<Arc<dyn File>, SvsmError>;
    fn create_directory(&self, name: FileName) -> Result<Arc<dyn Directory>, SvsmError>;
    fn unlink(&self, name: FileName) -> Result<(), SvsmError>;
}

pub enum DirEntry {
    File(Arc<dyn File>),
    Directory(Arc<dyn Directory>),
}

impl DirEntry {
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Directory(_))
    }
}

impl Clone for DirEntry {
    fn clone(&self) -> Self {
        match self {
            DirEntry::File(f) => DirEntry::File(f.clone()),
            DirEntry::Directory(d) => DirEntry::Directory(d.clone()),
        }
    }
}

pub struct DirectoryEntry {
    pub name: FileName,
    pub entry: DirEntry,
}

impl DirectoryEntry {
    pub fn new(name: FileName, entry: DirEntry) -> Self {
        DirectoryEntry { name, entry }
    }
}
