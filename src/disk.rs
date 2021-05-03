use crate::config::ENV_CONFIG;
use std::{
    convert::TryInto,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
};

pub struct DiskManager {
    pub heap_file: File,
    pub next_page_id: u64,
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / ENV_CONFIG.page_size as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
        Self::new(heap_file)
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        self.seek(page_id)?;
        self.heap_file.write_all(data)
    }
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        self.seek(page_id)?;
        self.heap_file.read_exact(data)
    }
    fn seek(&mut self, page_id: PageId) -> io::Result<()> {
        let offset = ENV_CONFIG.page_size * page_id.0;
        self.heap_file.seek(SeekFrom::Start(offset))?;
        Ok(())
    }
}

use zerocopy::{AsBytes, FromBytes};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromBytes, AsBytes, Default)]
#[repr(C)]
pub struct PageId(pub u64);

impl PageId {
    pub const INVALID_PAGE_ID: PageId = PageId(std::u64::MAX);
    pub fn valid(&self) -> Option<Self> {
        Some(PageId(self.0))
    }
}

// ここも取ってきた
impl From<&[u8]> for PageId {
    fn from(bytes: &[u8]) -> Self {
        let arr = bytes.try_into().unwrap();
        PageId(u64::from_ne_bytes(arr))
    }
}
impl From<Option<PageId>> for PageId {
    fn from(option: Option<PageId>) -> Self {
        option.unwrap_or_default()
    }
}
