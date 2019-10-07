#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! A Rust implementation of Fortune's Linesweep algorithm for computing Voronoi diagrams.

extern crate fnv;
extern crate log;
extern crate ordered_float;
extern crate rand;

mod beachline;
mod cell;
mod dcel;
mod event;
mod geometry;
mod lloyd;
mod point;
mod voronoi;

pub use crate::cell::Cell;
pub use crate::dcel::{make_line_segments, make_polygons, DCEL};
pub use crate::lloyd::{lloyd_relaxation, polygon_centroid};
pub use crate::point::Point;
pub use crate::voronoi::voronoi;
