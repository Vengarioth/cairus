/*
 * Cairus - a reimplementation of the cairo graphics library in Rust
 *
 * Copyright © 2017 CairusOrg
 *
 * This library is free software; you can redistribute it and/or
 * modify it either under the terms of the GNU Lesser General Public
 * License version 2.1 as published by the Free Software Foundation
 * (the "LGPL") or, at your option, under the terms of the Mozilla
 * Public License Version 2.0 (the "MPL"). If you do not alter this
 * notice, a recipient may use your version of this file under either
 * the MPL or the LGPL.
 *
 * You should have received a copy of the LGPL along with this library
 * in the file LICENSE-LGPL-2_1; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Suite 500, Boston, MA 02110-1335, USA
 * You should have received a copy of the MPL along with this library
 * in the file LICENSE-MPL-2_0
 *
 * The contents of this file are subject to the Mozilla Public License
 * Version 2.0 (the "License"); you may not use this file except in
 * compliance with the License. You may obtain a copy of the License at
 * http://www.mozilla.org/MPL/
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY
 * OF ANY KIND, either express or implied. See the LGPL or the MPL for
 * the specific language governing rights and limitations.
 *
 * The Original Code is the cairus graphics library.
 *
 * Contributor(s):
 *	Bobby Eshleman <bobbyeshleman@gmail.com>
 *
 */

//! This module defines geometric structs and methods common to algorithms used throughout Cairus.

use std::ops::{Add, Sub};
use std::f32;

/// ## Point
///
/// Defines a point by two floating points x and y.
 #[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point{x: self.x - other.x, y: self.y - other.y}
    }
}

/// ## LineSegment
///
/// Defines a line by two points.
#[derive(Debug, Copy, Clone)]
pub struct LineSegment {
    pub point1: Point,
    pub point2: Point,
}

impl LineSegment {
    // Returns a line.  Constructed by (x,y)-coordinates of two points.
    pub fn new(first_x: f32, first_y: f32, second_x: f32, second_y: f32) -> LineSegment {
        LineSegment {
            point1: Point{x: first_x, y: first_y},
            point2: Point{x: second_x, y: second_y}
        }
    }

    // Returns a line.  Constructed from two points.
    pub fn from_points(point1: Point, point2: Point) -> LineSegment {
        LineSegment {
            point1: point1,
            point2: point2,
        }
    }

    // Returns the length of this LineSegment
    pub fn length(&self) -> f32 {
        (self.point2.x - self.point1.x + self.point2.y - self.point1.y).sqrt()
    }

    /// Returns the slope of this LineSegment.
    ///
    /// If the slope is completely vertical, this function will return f32::INFINITY, otherwise
    /// it will return any valid f32 (assuming valid points form this LineSegment).
    ///
    /// One of the ways Cairo C implements slope comparision is using the following formula:
    ///     `(adx * bdy) ? (bdx * ady)`, where `?` is the comparison operator.
    ///
    /// Using this equation, any line with a slope of `delta x == 0` (divide by zero for the
    /// common rise/run slope equation) will zero out one side of the equation.  This means that
    /// any vertical line has a greater slope than any other non-vertical line.
    ///
    /// Fortunately, this logic is exactly equivalent to Rust's f32 implementation, and so the following
    /// slope implementation simply leverages f32's native comparison operations.  The only change
    /// is to make negative infinity a positive infinity, so that all vertical lines have equal
    /// slope, regardless of the direction from point1 to point2.
    pub fn slope(&self) -> f32 {
        let delta_x = self.point2.x - self.point1.x;
        let delta_y = self.point2.y - self.point1.y;
        let result = delta_y / delta_x;

        // Slope of negative infinity should be equal to positive infinity.
        if result.is_infinite() && result.is_sign_negative() {
            f32::INFINITY
        } else {
            result
        }
    }

    // Returns a Point, the midpoint between the two endpoints of self.
    pub fn midpoint(&self) -> Point {
        Point {
            x: (self.point1.x + self.point2.x) / 2.,
            y: (self.point1.y + self.point2.y) / 2.,
        }
    }

    pub fn highest_point(&self) -> Point {
        if self.point1.y > self.point2.y {
            self.point1
        } else {
            self.point2
        }
    }

    pub fn lowest_point(&self) -> Point {
        if self.point1.y < self.point2.y {
            self.point1
        } else {
            self.point2
        }
    }

    pub fn leftmost_point(&self) -> Point {
        if self.point1.x < self.point2.x {
            self.point1
        } else {
            self.point2
        }
    }

    pub fn rightmost_point(&self) -> Point {
        if self.point1.x > self.point2.x {
            self.point1
        } else {
            self.point2
        }
    }


    // Returns a Vector of coordinates indicating which pixels this line should color when
    // rasterized.  The algorithm is a straight-forward DDA.
    pub fn into_pixel_coordinates(&self) -> Vec<(i32, i32)> {
        let leftpoint = self.leftmost_point();
        let rightpoint = self.rightmost_point();

        let delta_x = rightpoint.x - leftpoint.x;
        let delta_y = rightpoint.y - leftpoint.y;

        let steps = if delta_x.abs() > delta_y.abs() {
            delta_x.abs()
        } else {
            delta_y.abs()
        };

        let x_increment = delta_x / steps;
        let y_increment = delta_y / steps;
        let (mut x, mut y) = (leftpoint.x, leftpoint.y);

        let mut result = Vec::with_capacity(steps as usize);
        for _ in 0..(steps as i32) {
            x += x_increment;
            y += y_increment;
            let point = (x as i32 , y as i32);
            result.push(point);
        }

        result
    }
}

impl PartialEq for LineSegment {
    fn eq(&self, other: &LineSegment) -> bool {
        (self.point1 == other.point1 && self.point2 == other.point2) ||
        (self.point1 == other.point2 && self.point2 == other.point1)
    }
}


/// ## Vector
///
/// Defines a vector by (x, y) direction.
#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector {
            x: x,
            y: y,
        }
    }

    // Returns the dot product of self and rhs.
    pub fn dot_product(&self, rhs: &Vector) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Returns the angle between self and rhs.
    pub fn angle_between(&self, rhs: &Vector) -> f32 {
        (
            self.dot_product(rhs) / (self.magnitude() * rhs.magnitude())
        ).acos()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::{LineSegment, Point, Vector};

    // Tests that point subtraction is working.
    #[test]
    fn point_subtraction() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        assert_eq!(p1 - p2, Point{x: -1., y: -1.});
    }

    // Tests that LineSegment's constructor is working.
    #[test]
    fn line_new() {
        let line = LineSegment::new(0., 0., 1., 1.);
        assert_eq!(line.point1, Point{x: 0., y: 0.});
        assert_eq!(line.point2, Point{x: 1., y: 1.});
    }

    // Tests that LineSegment's `from_points` alternative constructor is working
    #[test]
    fn line_from_points() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        let line = LineSegment::from_points(p1, p2);
        assert_eq!(line.point1, Point{x: 0., y: 0.});
        assert_eq!(line.point2, Point{x: 1., y: 1.});
    }

    // Tests that LineSegment's  highest/lowest/leftmost/rightmost point functions work
    #[test]
    fn line_query_functions() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        let line = LineSegment::from_points(p1, p2);
        assert_eq!(line.leftmost_point(), p1);
        assert_eq!(line.lowest_point(), p1);
        assert_eq!(line.rightmost_point(), p2);
        assert_eq!(line.highest_point(), p2);
    }

    // Tests that LineSegment Eq implementation is working
    #[test]
    fn line_eq() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        let line1 = LineSegment::from_points(p1, p2);
        let line2 = LineSegment::from_points(p1, p2);

        assert_eq!(line1, line2);
    }

    // Tests that lines are equal even when the endpoints are swithced
    #[test]
    fn line_eq_opposite() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        let line1 = LineSegment::from_points(p1, p2);
        let line2 = LineSegment::from_points(p2, p1);

        assert_eq!(line1, line2);
    }

    // Tests that the simple case for LineSegment::slope() is working.
    #[test]
    fn line_slope() {
        let line = LineSegment::new(0., 0., 1., 1.);
        assert_eq!(line.slope(), 1.);
    }

    // Tests that the simple case for LineSegment::midpoint() is working.
    #[test]
    fn line_midpoint() {
        let line = LineSegment::new(0., 0., 2., 2.);
        assert_eq!(line.midpoint(), Point{x: 1., y: 1.});
    }

    // Tests that LineSegment::midpoint() is working when point2's x-value is less than point1's.
    #[test]
    fn line_opposite_direction_midpoint() {
        let line = LineSegment::new(2., 2., 0., 0.);
        assert_eq!(line.midpoint(), Point{x: 1., y: 1.});
    }

    // Tests that midpoint works for lines with negative slope
    #[test]
    fn line_negative_slope_midpoint() {
        let line = LineSegment::new(0., 0., 2., -2.);
        assert_eq!(line.midpoint(), Point{x: 1., y: -1.});
    }

    // Tests that midpoint works for vertical lines
    #[test]
    fn vertical_line_midpoint() {
        let line = LineSegment::new(0., 0., 0., 2.);
        assert_eq!(line.midpoint(), Point{x: 0., y: 1.});
    }

    // Tests that midpoint works for negative vertical lines
    #[test]
    fn vertical_negative_slope_midpoint() {
        let line = LineSegment::new(0., 0., 0., -2.);
        assert_eq!(line.midpoint(), Point{x: 0., y: -1.});
    }

    // Tests greater than slope comparison
    #[test]
    fn vertical_slope_gt_positive() {
        let vertical = LineSegment::new(0., 0., 0., 1.);
        let positive = LineSegment::new(0., 0., 1., 1.);
        assert!(vertical.slope() > positive.slope());
    }

    // Tests greater than slope comparison with one negative slope
    #[test]
    fn vertical_slope_gt_negative() {
        let vertical = LineSegment::new(0., 0., 0., 1.);
        let negative = LineSegment::new(0., 0., 1., -1.);
        assert!(vertical.slope() > negative.slope());
    }

    // Tests equality of slopes
    #[test]
    fn vertical_slope_eq_vertical() {
        let vertical1 = LineSegment::new(0., 0., 0., 1.);
        let vertical2 = LineSegment::new(2., 2., 2., -1.);
        assert_eq!(vertical1.slope(), vertical2.slope());
    }

    // Tests Vector::new()
    #[test]
    fn vector_new() {
        let vec = Vector::new(1., 1.);
        assert_eq!(vec.x, 1.);
        assert_eq!(vec.y, 1.);
    }

    // Tests overloaded Vector addition operator
    #[test]
    fn vector_add() {
        let a = Vector::new(0., 0.);
        let b = Vector::new(1., 1.);
        let c = a + b;
        assert_eq!(c, b);
    }

    // Tests Vector::dot_product()
    #[test]
    fn vector_dot_product() {
        let a = Vector::new(1., 0.);
        let b = Vector::new(1., 1.);
        let c = a.dot_product(&b);
        assert_eq!(c, 1.);
    }

    // Tests Vector::magnitude()
    #[test]
    fn vector_magnitude() {
        let b = Vector::new(3., 4.);
        assert_eq!(b.magnitude(), 5.);
    }

    // Tests Vector::angle_between()
    #[test]
    fn vector_angle_between() {
        let a = Vector::new(1., 0.);
        let b = Vector::new(1., 1.);
        assert_eq!(a.angle_between(&b).to_degrees(), 45.)
    }

    #[test]
      fn line_into_pixel_coordinates_slope_lt_one() {
          // The following coordinates were calculated by hand to be known pixels in the defined
          // line.
          let line = LineSegment::new(0., 0., 20., 5.);
          let expected = vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 1),
            (5, 1),
            (6, 1)
          ];

          let pixel_coordinates = line.into_pixel_coordinates();
          for coordinate in expected {
              assert!(pixel_coordinates.contains(&coordinate));
          }
      }

      #[test]
      fn line_into_pixel_coordinates_slope_gt_one() {
          // The following coordinates were calculated by hand to be known pixels in the defined
          // line.
          let line = LineSegment::new(0., 0., 5., 20.);
          let expected = vec![
              (0, 1),
              (0, 2),
              (0, 3),
              (1, 4),
              (1, 5),
              (1, 6),
              (1, 7)
          ];

          let pixel_coordinates = line.into_pixel_coordinates();
          for coordinate in expected {
              assert!(pixel_coordinates.contains(&coordinate));
          }
      }


      #[test]
      fn line_with_negative_slope() {
          let line = LineSegment { point1: Point { x: 3., y: 2. }, point2: Point { x: 4., y: 0. } };
          for pixel in line.into_pixel_coordinates() {
              let x = pixel.0;
              let y = pixel.1;
              assert!(y >= 0);
              assert!(x >= 0);
          }
      }

      // Passes if LineSegment::length() works
      #[test]
      fn line_length() {
          let line = LineSegment::new(0., 0., 2., 2.);
          assert_eq!(line.length(), 2.);
      }

}
///SplineKnots for bezier curves
pub struct SplineKnots{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

///Implements SplineKnots methods
impl SplineKnots{
///Creates a new SplineKnots with user defined points
    fn create(a: &Point, b: &Point, c: &Point, d: &Point)->SplineKnots{
        SplineKnots{
            a:Point::create(a.x, a.y),
            b:Point::create(b.x, b.y),
            c:Point::create(c.x, c.y),
            d:Point::create(d.x, d.y),
        }
    }
}

