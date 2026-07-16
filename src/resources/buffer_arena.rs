use std::u32;


#[derive(Clone, Copy)]
pub struct RangeInfo {
    pub start: u32,
    pub len: u32
}

impl RangeInfo {
    pub const EMPTY: Self  = Self { start: 0, len: 0 };

    pub fn new(start: u32, len: u32) -> Self { Self { start, len } }

    pub fn end(self) -> u32 {
        self.start + self.len
    }

    pub fn is_empty(self) -> bool {
        self.start == 0 && self.len == 0
    }
}

pub struct BufferArena {
    capacity: u32,
    used: u32,
    range_margin: u32,

    free_blocks: Vec<RangeInfo>,
}

impl BufferArena {
    pub const MB: u32 = 1024 * 1024;
    pub const KB: u32 = 1024;

    pub fn new(capacity: u32, range_margin: u32) -> Self {
        Self {
            capacity: capacity,
            used: 0,
            range_margin: range_margin,

            free_blocks: vec![RangeInfo::new(0, capacity)],
        }
    }

    pub fn get_used(&self) -> u32 {
        self.used
    }

    pub fn restore_range(&mut self, range: &mut RangeInfo) {
        if range.is_empty() { return }

        let mut left_neighbor_idx: Option<usize> = None;
        let mut right_neighbor_idx: Option<usize> = None;

        for i in 0..self.free_blocks.len() {
            let free_range = self.free_blocks[i];

            if free_range.end() == range.start {
                left_neighbor_idx = Some(i);
            }
            else if range.end() == free_range.start {
                right_neighbor_idx = Some(i);
            }

            // we found all neightbors
            if left_neighbor_idx.is_some() && right_neighbor_idx.is_some() {
                break;
            }
        }

        // neighbors not found, then just push the range
        if left_neighbor_idx.is_none() && right_neighbor_idx.is_none() {
            self.free_blocks.push(*range);
        }
        else if let Some(left_idx) = left_neighbor_idx && let Some(right_idx) = right_neighbor_idx {
            // combine neighbors ranges (left and right) in just one range
            let left_range = self.free_blocks[left_idx];
            let right_range = self.free_blocks[right_idx];

            let combined_with_left = RangeInfo::new(left_range.start, left_range.len + range.len);
            let combined_ranges = RangeInfo::new(left_range.start, combined_with_left.len + right_range.len);

            self.free_blocks[left_idx] = combined_ranges;

            // we update the left neighbor, then we can remove the right
            self.free_blocks.swap_remove(right_idx);
        }
        else if let Some(left_idx) = left_neighbor_idx {
            let left_range = self.free_blocks[left_idx];

            self.free_blocks[left_idx] = RangeInfo::new(left_range.start, left_range.len + range.len);
        }
        else if let Some(right_idx) = right_neighbor_idx {
            let right_range = self.free_blocks[right_idx];

            self.free_blocks[right_idx] = RangeInfo::new(range.start, range.len + right_range.len);
        }

        self.used -= range.len;
        *range = RangeInfo::EMPTY;
    }

    pub fn find_range(&mut self, size: u32) -> Option<RangeInfo> {
        let range = self.find_range_with_margin(size);

        if let Some(range) = range {
            self.used += range.len;
        }

        return range;
    }

    pub fn refind_range(&mut self, range: &mut RangeInfo, new_size: u32) -> Result<(), ()> {
        assert!(new_size != 0 || new_size > self.capacity, "Invalid size");

        if range.len >= new_size {
            return Ok(());
        }

        self.restore_range(range);

        return match self.find_range(new_size) {
            Some(new_range) => {
                *range = new_range;
                Ok(())
            },
            None => Err(())
        };
    }

    fn find_range_with_margin(&mut self, size: u32) -> Option<RangeInfo> {
        assert!(size != 0 || size > self.capacity, "Invalid size");

        let mut smallest_idx: Option<usize> = None;
        let mut smallest_size = u32::MAX;

        for i in 0..self.free_blocks.len() {
            let current = self.free_blocks[i];

            if current.len >= size && current.len < smallest_size {
                smallest_idx = Some(i);
                smallest_size = current.len;
            }
        }

        if let Some(idx) = smallest_idx {
            let free_range = self.free_blocks[idx];

            // we consume all range, then remove it from free blocks
            if free_range.len == size {
                self.free_blocks.swap_remove(idx);

                return Some(free_range);
            }
            else {
                let mut range = RangeInfo::new(free_range.start, size);

                // if range len are inside of acceptable margin, then we consume all range
                if free_range.len - size <= self.range_margin {
                    range = free_range;
                    self.free_blocks.swap_remove(idx);
                }
                else {
                    self.free_blocks[idx] = RangeInfo::new(range.end(), free_range.len - size);
                }

                return Some(range);
            }
        }

        return None;
    }
}
