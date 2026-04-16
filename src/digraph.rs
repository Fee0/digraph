//! Core digraph type and construction.

use crate::accumulate::{self, StreamState};
use std::io::{Read, Result as IoResult};

/// How consecutive bytes are grouped into pairs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Mode {
    /// Pairs `(b[i], b[i+1])`, `(b[i+1], b[i+2])`, …
    Overlapping,
    /// Pairs `(b[0], b[1])`, `(b[2], b[3])`, …
    NonOverlapping,
}

/// 256×256 histogram of byte bigrams (first byte = X, second = Y).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Digraph {
    counts: Vec<u32>,
    max_count: u32,
}

impl Default for Digraph {
    fn default() -> Self {
        Self::empty()
    }
}

impl Digraph {
    const LEN: usize = 256 * 256;

    /// All-zero digraph.
    pub fn empty() -> Self {
        Self {
            counts: vec![0; Self::LEN],
            max_count: 0,
        }
    }

    /// Builds a digraph from a byte slice using [`Mode::Overlapping`].
    pub fn from_bytes(data: &[u8]) -> Self {
        Self::from_bytes_with_mode(data, Mode::Overlapping)
    }

    /// Builds a digraph from a byte slice.
    pub fn from_bytes_with_mode(data: &[u8], mode: Mode) -> Self {
        let mut d = Self::empty();
        d.add_bytes_with_mode(data, mode);
        d
    }

    /// Reads the entire stream into memory, then builds a digraph.
    pub fn from_reader<R: Read>(mut reader: R, mode: Mode) -> IoResult<Self> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(Self::from_bytes_with_mode(&buf, mode))
    }

    /// Adds counts from `data` using [`Mode::Overlapping`].
    pub fn add_bytes(&mut self, data: &[u8]) {
        self.add_bytes_with_mode(data, Mode::Overlapping);
    }

    /// Adds counts from `data` using the given mode (independent of any prior `add_*` calls).
    pub fn add_bytes_with_mode(&mut self, data: &[u8], mode: Mode) {
        match mode {
            Mode::Overlapping => {
                accumulate::accumulate_overlapping(&mut self.counts, &mut self.max_count, data)
            }
            Mode::NonOverlapping => {
                accumulate::accumulate_non_overlapping(&mut self.counts, &mut self.max_count, data)
            }
        }
    }

    /// Number of cells (always 65_536).
    #[inline]
    pub fn len(&self) -> usize {
        Self::LEN
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Raw counts in row-major order: index `(x << 8) | y` for pair `(x, y)`.
    #[inline]
    pub fn counts(&self) -> &[u32] {
        &self.counts
    }

    /// Largest cell value after construction / additions.
    #[inline]
    pub fn max_count(&self) -> u32 {
        self.max_count
    }

    #[inline]
    pub fn get(&self, x: u8, y: u8) -> u32 {
        self.counts[accumulate::index(x, y)]
    }

    /// Recomputes `max_count` from cells (e.g. after manual edits).
    pub fn refresh_max_count(&mut self) {
        self.max_count = self.counts.iter().copied().max().unwrap_or(0);
    }
}

#[cfg(feature = "image")]
impl Digraph {
    /// Renders this digraph as an RGBA raster (`feature = "image"`).
    pub fn to_rgba(&self, params: crate::render::RenderParams) -> image::RgbaImage {
        crate::render::render_rgba(self, params)
    }

    /// Writes a PNG (`feature = "image"`).
    pub fn to_png<W: std::io::Write + std::io::Seek>(
        &self,
        params: crate::render::RenderParams,
        writer: W,
    ) -> Result<(), crate::RenderError> {
        crate::render::render_png(self, params, writer)
    }
}

#[cfg(feature = "svg")]
impl Digraph {
    /// Renders a heatmap as SVG (`feature = "svg"`).
    pub fn to_svg_heatmap(&self, params: crate::render::SvgParams) -> String {
        crate::render::render_svg_heatmap(self, params)
    }
}

/// Incremental digraph builder that keeps pair continuity across chunk boundaries.
#[derive(Debug)]
pub struct DigraphBuilder {
    digraph: Digraph,
    stream: StreamState,
}

impl DigraphBuilder {
    pub fn new(mode: Mode) -> Self {
        Self {
            digraph: Digraph::empty(),
            stream: StreamState::new(mode),
        }
    }

    pub fn mode(&self) -> Mode {
        self.stream.mode
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.stream
            .push(&mut self.digraph.counts, &mut self.digraph.max_count, bytes);
    }

    pub fn finish(self) -> Digraph {
        self.digraph
    }

    pub fn digraph_ref(&self) -> &Digraph {
        &self.digraph
    }
}
