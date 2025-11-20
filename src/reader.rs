use crate::config::{APP_CACHE_SIZE, APP_PAGE_SIZE};

/// Reader is the class that implements the file
/// reader and buffering. It reads the file in blocks
/// of APP_CACHE_SIZE, but keep track of the page
/// so drawing functions in hex/draw.rs can render
/// a subset of what's in the cache to speed things up.

#[derive(Default, Debug)]
pub struct Reader {
    pub cache_block_number: usize,
    pub cache_blocks: usize,
    pub cache_start: usize,
    pub cache_end: usize,
    pub offset_location_in_cache: usize,
    pub page_current_size: usize,
    pub page_current: usize,
    pub page_last: usize,
    pub page_start: usize,
    pub page_end: usize,
    pub pages: usize,
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            cache_end: APP_CACHE_SIZE - 1,
            page_end: APP_PAGE_SIZE - 1,
            ..Default::default()
        }
    }
}
