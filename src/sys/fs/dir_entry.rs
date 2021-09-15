use super::{dirname, filename, realpath, FileType};
use super::dir::Dir;
use alloc::string::String;

#[derive(Clone)]
pub struct DirEntry {
    dir: Dir,
    kind: FileType,
    addr: u32,
    size: u32,
    time: u64,
    name: String,
}

impl DirEntry {
    pub fn open(pathname: &str) -> Option<Self> {
        let pathname = realpath(pathname);
        let dirname = dirname(&pathname);
        let filename = filename(&pathname);
        if let Some(dir) = Dir::open(dirname) {
            return dir.find(filename);
        }
        None
    }

    pub fn new(dir: Dir, kind: FileType, addr: u32, size: u32, time: u64, name: &str) -> Self {
        let name = String::from(name);
        Self { dir, kind, addr, size, time, name }
    }

    pub fn empty_len() -> usize {
        1 + 4 + 4 + 8 + 1
    }

    pub fn len(&self) -> usize {
        Self::empty_len() + self.name.len()
    }

    pub fn is_empty(&self) -> bool {
        Self::empty_len() == self.len()
    }

    pub fn kind(&self) -> FileType {
        self.kind
    }

    pub fn is_dir(&self) -> bool {
        self.kind == FileType::Dir
    }

    pub fn is_file(&self) -> bool {
        self.kind == FileType::File
    }

    pub fn is_device(&self) -> bool {
        self.kind == FileType::Device
    }

    pub fn addr(&self) -> u32 {
        self.addr
    }

    pub fn dir(&self) -> Dir {
        self.dir
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn stat(&self) -> FileStat {
        FileStat { kind: self.kind, size: self.size, time: self.time }
    }
}

#[derive(Debug)]
pub struct FileStat {
    kind: FileType,
    size: u32,
    time: u64,
}

impl FileStat {
    pub fn new() -> Self {
        Self { kind: FileType::File, size: 0, time: 0 }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    // TODO: Duplicated from dir entry
    pub fn is_dir(&self) -> bool {
        self.kind == FileType::Dir
    }

    pub fn is_file(&self) -> bool {
        self.kind == FileType::File
    }

    pub fn is_device(&self) -> bool {
        self.kind == FileType::Device
    }
}
