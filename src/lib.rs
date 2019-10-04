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

#[macro_use]
extern crate log;
extern crate fnv;
extern crate ordered_float;
extern crate rand;

mod beachline;
mod dcel;
mod event;
mod geometry;
mod lloyd;
mod point;
mod voronoi;

pub use dcel::{make_line_segments, make_polygons, DCEL};
pub use lloyd::{lloyd_relaxation, polygon_centroid};
pub use point::Point;
pub use voronoi::voronoi;
