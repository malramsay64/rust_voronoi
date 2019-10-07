//
// cell.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use crate::Point;

fn intersect_point(point: &Point, start: &Point, finish: &Point) -> bool {
    // This compares the squared distance of a point to the start and finish,
    // with the distance from the start to the finish

    let d_ps_2 = (point.x() - start.x()).powi(2) + (point.y() - start.y()).powi(2);
    let d_pf_2 = (point.x() - finish.x()).powi(2) + (point.y() - finish.y()).powi(2);
    let d_sf_2 = (start.x() - finish.x()).powi(2) + (start.y() - finish.y()).powi(2);

    d_ps_2.sqrt() + d_pf_2.sqrt() - d_sf_2.sqrt() < 2. * std::f64::EPSILON
}

fn intersects(s1: &Point, f1: &Point, s2: &Point, f2: &Point) -> bool {
    // Also see below links for other implementations of this algorithm
    // - https://github.com/georust/geo/blob/96c7846d703a74f59ba68e68929415cbce4a68d9/geo/src/algorithm/intersects.rs#L142
    // - https://github.com/brandonxiang/geojson-python-utils/blob/33b4c00c6cf27921fb296052d0c0341bd6ca1af2/geojson_utils.py
    // - http://www.kevlindev.com/gui/math/intersection/Intersection.js
    //
    let u_b = (f2.y() - s2.y()) * (f1.x() - s1.x()) - (f2.x() - s2.x()) * (f1.y() - s1.y());
    // Where u_b == 0 the two lines are parallel. In this case we don't need any further checks
    // since we are only concerned with lines that cross, parallel is fine.
    if u_b == 0. {
        return false;
    }

    let ua_t = (f2.x() - s2.x()) * (s1.y() - s2.y()) - (f2.y() - s2.y()) * (s1.x() - s2.x());
    let ub_t = (f1.x() - s1.x()) * (s1.y() - s2.y()) - (f1.y() - s1.y()) * (s1.x() - s2.x());

    let ua = ua_t / u_b;
    let ub = ub_t / u_b;
    // Should the points ua, ub both lie on the interval [0, 1] the lines intersect.
    if 0. <= ua && ua <= 1. && 0. <= ub && ub <= 1. {
        return true;
    }
    false
}

/// Define the bounding box for the voronoi diagram
///
/// This specifies the corners of the voronoi cell.
#[derive(Debug, Clone)]
pub struct Cell {
    boundary: [Point; 4],
}

impl From<[Point; 4]> for Cell {
    fn from(boundary: [Point; 4]) -> Cell {
        Cell { boundary }
    }
}

impl Cell {
    /// Create a Cell from the size of a box
    ///
    /// This creates a Cell instance which starts at the origin and extends `boxsize` units in the
    /// positive x and y directions.
    ///
    pub fn new(boxsize: f64) -> Cell {
        Cell {
            boundary: [
                Point::new(0., 0.),
                Point::new(boxsize, 0.),
                Point::new(boxsize, boxsize),
                Point::new(0., boxsize),
            ],
        }
    }

    pub(crate) fn top(&self) -> [Point; 2] {
        [self.boundary[2], self.boundary[3]]
    }

    pub(crate) fn bottom(&self) -> [Point; 2] {
        [self.boundary[0], self.boundary[1]]
    }

    pub(crate) fn left(&self) -> [Point; 2] {
        [self.boundary[3], self.boundary[0]]
    }

    pub(crate) fn right(&self) -> [Point; 2] {
        [self.boundary[1], self.boundary[2]]
    }

    pub(crate) fn contains(&self, point: &Point) -> bool {
        // If the point is the same as a boundary it is contained
        if self.boundary.iter().any(|x| x == point) {
            return true;
        }
        // If the point lies on the right boundary then we include it
        if [self.top(), self.bottom(), self.left(), self.right()]
            .iter()
            .map(|l| intersect_point(point, &l[0], &l[1]))
            .any(|b| b)
        {
            return true;
        }
        let intersections = [self.top(), self.bottom(), self.left(), self.right()]
            .iter()
            .map(|l| intersects(point, &(self.boundary[2] + self.boundary[1]), &l[0], &l[1]))
            .filter(|&b| b)
            .count();
        intersections % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_start() {
        assert!(intersects(
            &Point::new(0., 0.),
            &Point::new(1., 0.),
            &Point::new(0., 0.),
            &Point::new(0., 1.)
        ))
    }

    #[test]
    fn intersect_middle_y() {
        assert!(intersects(
            &Point::new(0., 0.),
            &Point::new(0., 1.),
            &Point::new(0., 0.5),
            &Point::new(1., 0.5)
        ))
    }

    #[test]
    fn intersect_middle_800() {
        assert!(intersects(
            &Point::new(801., 0.),
            &Point::new(801., 800.),
            &Point::new(800., 15.0),
            &Point::new(810., 15.0)
        ))
    }

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
}
