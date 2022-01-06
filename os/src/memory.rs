
#[derive(Debug, Clone)]
pub struct MemoryArea {
    start_addr: VirtAddr,
    end_addr: VirtAddr,
    attr: MemoryAttr,
    handler: Box<dyn MemoryHandler>,
    name: &'static str,
}

impl MemoryArea {
    /// Test whether a virtual address is in the memory area
    pub fn contains(&self, addr: VirtAddr) -> bool {
        addr >= self.start_addr && addr < self.end_addr
    }
    /// Check the array is within the readable memory.
    /// Return the size of space covered in the area.
    fn check_read_array<S>(&self, ptr: *const S, count: usize) -> usize {
        // page align
        let min_bound = (ptr as usize).max(Page::of_addr(self.start_addr).start_address());
        let max_bound = unsafe { ptr.add(count) as usize }
            .min(Page::of_addr(self.end_addr + PAGE_SIZE - 1).start_address());
        if max_bound >= min_bound {
            max_bound - min_bound
        } else {
            0
        }
    }
    /// Check the array is within the writable memory.
    /// Return the size of space covered in the area.
    fn check_write_array<S>(&self, ptr: *mut S, count: usize) -> usize {
        if self.attr.readonly {
            0
        } else {
            self.check_read_array(ptr, count)
        }
    }
    /// Test whether this area is (page) overlap with area [`start_addr`, `end_addr`)
    pub fn is_overlap_with(&self, start_addr: VirtAddr, end_addr: VirtAddr) -> bool {
        let p0 = Page::of_addr(self.start_addr);
        let p1 = Page::of_addr(self.end_addr - 1) + 1;
        let p2 = Page::of_addr(start_addr);
        let p3 = Page::of_addr(end_addr - 1) + 1;
        !(p1 <= p2 || p0 >= p3)
    }
    /// Map all pages in the area to page table `pt`
    fn map(&self, pt: &mut dyn PageTable) {
        for page in Page::range_of(self.start_addr, self.end_addr) {
            self.handler.map(pt, page.start_address(), &self.attr);
        }
    }
    /// Unmap all pages in the area from page table `pt`
    fn unmap(&self, pt: &mut dyn PageTable) {
        for page in Page::range_of(self.start_addr, self.end_addr) {
            self.handler.unmap(pt, page.start_address());
        }
    }
}