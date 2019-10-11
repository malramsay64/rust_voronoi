//
// cell.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use crate::Point;
use anyhow::anyhow;
use std::cmp::Ordering;
use std::convert::TryFrom;

/// Define the bounding box for the voronoi diagram
///
/// This specifies the corners of the voronoi cell.
#[derive(Debug, Clone)]
pub struct Cell {
    boundary: Vec<Point>,
}

impl TryFrom<Vec<Point>> for Cell {
    type Error = anyhow::Error;
    fn try_from(boundary: Vec<Point>) -> Result<Cell, Self::Error> {
        if boundary.len() < 3 {
            return Err(anyhow!(
                "Not enough points for a contained shape, found {} require 3",
                boundary.len()
            ));
        }

        Ok(Cell { boundary })
    }
}

#[inline]
fn point_to_line(p: &Point, l0: &Point, l1: &Point) -> Option<Ordering> {
    ((l1.x() - l0.x()) * (p.y() - l0.y()) - (p.x() - l0.x()) * (l1.y() - l0.y())).partial_cmp(&0.)
}

impl Cell {
    /// Create a Cell from the size of a box
    ///
    /// This creates a Cell instance which starts at the origin and extends `boxsize` units in the
    /// positive x and y directions.
    ///
    pub fn new(boxsize: f64) -> Cell {
        Cell {
            boundary: vec![
                Point::new(0., 0.),
                Point::new(boxsize, 0.),
                Point::new(boxsize, boxsize),
                Point::new(0., boxsize),
            ],
        }
    }

    pub(crate) fn sides(&self) -> impl Iterator<Item = (&Point, &Point)> {
        self.boundary
            .iter()
            .zip(self.boundary.iter().cycle().skip(1))
    }

    /// Find the area of the cell
    pub(crate) fn area(&self) -> f64 {
        self.sides()
            .map(|(curr, next)| (next.x() + curr.x()) * (next.y() - curr.y()))
            .sum::<f64>()
            .abs()
            / 2.
    }

    pub(crate) fn contains(&self, point: &Point) -> bool {
        let mut acc = 0;
        for (v0, v1) in self.sides() {
            // Check whether point overlaps the line
            match (v0.y.cmp(&point.y), point.y.cmp(&v1.y)) {
                // The line has an upwards direction
                // start <= point <= end
                (Ordering::Less, Ordering::Less)
                | (Ordering::Equal, Ordering::Less)
                | (Ordering::Less, Ordering::Equal) => match point_to_line(point, v0, v1) {
                    // The point is on the
                    Some(Ordering::Less) => acc += 1,
                    // The point is on the line so exit early
                    Some(Ordering::Equal) => return true,
                    Some(Ordering::Greater) => {}
                    // There are values which can't be compared (NaN, Inf) so the point is not
                    // contained
                    None => return false,
                },
                // The line has a downwards direction
                // start >= point >= end
                (Ordering::Greater, Ordering::Greater)
                | (Ordering::Greater, Ordering::Equal)
                | (Ordering::Equal, Ordering::Greater) => match point_to_line(point, v0, v1) {
                    // The point is on the right, which is the left since this line is going down
                    Some(Ordering::Greater) => acc -= 1,
                    // The point is on the line so exit early
                    Some(Ordering::Equal) => return true,
                    Some(Ordering::Less) => {}
                    // There are values which can't be compared (NaN, Inf) so the point is not
                    // contained
                    None => return false,
                },
                _ => {}
            }
        }
        acc % 2 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::proptest;

    #[test]
    fn containment_simple() {
        let cell = Cell::new(10.);
        assert!(cell.contains(&Point::new(1., 1.)))
    }

    #[test]
    fn containment_corners() {
        let cell = Cell::new(10.);
        assert!(cell.contains(&Point::new(0., 0.)));
        assert!(cell.contains(&Point::new(10., 10.)));
    }

    #[test]
    fn containment_edge() {
        let cell = Cell::new(10.);
        assert!(cell.contains(&Point::new(0., 5.)));
        assert!(cell.contains(&Point::new(10., 5.)));
    }

    #[test]
    fn outside_x() {
        let cell = Cell::new(10.);
        assert!(!cell.contains(&Point::new(-1., 5.)));
        assert!(!cell.contains(&Point::new(-10., 5.)));
    }

    #[test]
    fn outside_y() {
        let cell = Cell::new(10.);
        assert!(!cell.contains(&Point::new(1., -5.)));
        assert!(!cell.contains(&Point::new(10., -5.)));
    }

    proptest! {
        #[test]
        fn rand_containment(x in 0_f64..1_f64, y in 0_f64..1_f64) {
            let cell = Cell::new(1.);
            assert!(cell.contains(&Point::new(x, y)));
        }
    }
    proptest! {
        #[test]
        fn rand_containment_false(x in 0_f64..1_f64, y in 0_f64..1_f64) {
            let cell = Cell::new(1.-std::f64::EPSILON);
            assert!(!cell.contains(&Point::new(x-1., y-1.)));
            assert!(!cell.contains(&Point::new(x, y-1.)));
            assert!(!cell.contains(&Point::new(x+1., y-1.)));

            assert!(!cell.contains(&Point::new(x-1., y)));
            assert!(!cell.contains(&Point::new(x+1., y)));

            assert!(!cell.contains(&Point::new(x-1., y+1.)));
            assert!(!cell.contains(&Point::new(x, y+1.)));
            assert!(!cell.contains(&Point::new(x+1., y+1.)));
        }
    }
}
