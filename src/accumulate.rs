use crate::digraph::Mode;

#[inline]
pub(crate) fn index(x: u8, y: u8) -> usize {
    (x as usize) << 8 | y as usize
}

pub(crate) fn accumulate_overlapping(counts: &mut [u32], max_count: &mut u32, bytes: &[u8]) {
    if bytes.len() < 2 {
        return;
    }
    let mut i = 0;
    while i + 1 < bytes.len() {
        let idx = index(bytes[i], bytes[i + 1]);
        let c = counts[idx].saturating_add(1);
        counts[idx] = c;
        *max_count = (*max_count).max(c);
        i += 1;
    }
}

pub(crate) fn accumulate_non_overlapping(counts: &mut [u32], max_count: &mut u32, bytes: &[u8]) {
    let mut i = 0;
    while i + 1 < bytes.len() {
        let idx = index(bytes[i], bytes[i + 1]);
        let c = counts[idx].saturating_add(1);
        counts[idx] = c;
        *max_count = (*max_count).max(c);
        i += 2;
    }
}

/// Feeds bytes into a digraph while preserving pair boundaries across chunks.
#[derive(Debug)]
pub(crate) struct StreamState {
    pub mode: Mode,
    overlapping_prev: Option<u8>,
    non_overlap_pending: Option<u8>,
}

impl StreamState {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            overlapping_prev: None,
            non_overlap_pending: None,
        }
    }

    pub fn push(&mut self, counts: &mut [u32], max_count: &mut u32, bytes: &[u8]) {
        match self.mode {
            Mode::Overlapping => {
                if let Some(prev) = self.overlapping_prev.take() {
                    if bytes.is_empty() {
                        self.overlapping_prev = Some(prev);
                        return;
                    }
                    let idx = index(prev, bytes[0]);
                    let c = counts[idx].saturating_add(1);
                    counts[idx] = c;
                    *max_count = (*max_count).max(c);
                    accumulate_overlapping(counts, max_count, bytes);
                    self.overlapping_prev = Some(*bytes.last().expect("non-empty"));
                } else {
                    accumulate_overlapping(counts, max_count, bytes);
                    self.overlapping_prev = bytes.last().copied();
                }
            }
            Mode::NonOverlapping => {
                let mut b = bytes;
                if let Some(hi) = self.non_overlap_pending.take() {
                    if b.is_empty() {
                        self.non_overlap_pending = Some(hi);
                        return;
                    }
                    let idx = index(hi, b[0]);
                    let c = counts[idx].saturating_add(1);
                    counts[idx] = c;
                    *max_count = (*max_count).max(c);
                    b = &b[1..];
                }
                accumulate_non_overlapping(counts, max_count, b);
                if b.len() % 2 == 1 {
                    self.non_overlap_pending = Some(*b.last().expect("non-empty"));
                } else {
                    self.non_overlap_pending = None;
                }
            }
        }
    }
}
