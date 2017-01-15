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
 *  Bobby Eshleman <bobbyeshleman@gmail.com>
 *
 */

//! This module defines image compositing operations.
//!
//! # Supported Operators:
//! * Over - Cairus's default operator.  Blends a source onto a destination, similar to overlapping
//!          two semi-transparent slides.  If the source is opaque, the over operation will make
//!          the destination opaque as well.
//!
//! Descriptions/formulas for Cairo operators:  [Cairo Operators](https://www.cairographics.org/operators/)
use types::Rgba;

// Image Compositing Operations
// This section defines all functions and enums for image compositing.
//
// Adding a new operator
// To add a new operator, implement the function for the operator, create an enum for it, and then
// add the "enum => function" match in `fetch_operator`.  The new operator will now be available
// to any context via `fetch_operator`.

/// The supported image compositing operators in Cairus.
pub enum Operator {
    /// Cairus's default operator.  Draws source layer on top of destination layer.
    Over,
}

/// Returns an image compositing function that corresponds to an Operator enum.
///
/// This function maps an enum to its function, allowing for dynamic determination of the operator
/// function.  This is likely a good way for a context to fetch the correct function just by having
/// an Operator enum.
///
/// # Arguments
/// * `op` - Reference to an enum `Operator` that matches the desired operation.
///
/// # Usage
/// let op_enum = Operator::Over;
///
/// // Fetch and use the operator
/// let compose = fetch_operator(&op_enum);
/// compose(&source, &mut destination1);
fn fetch_operator(op: &Operator) -> fn(&Rgba, &mut Rgba) {
    match *op {
        Operator::Over => over,
    }
}

/// Composites `source` over `destination`.
///
/// # Arguments
/// * `source` - The source Rgba to be applied to the destination Rgba.
/// * `destination` - The destination Rgba that holds the resulting composition.
///
/// Over is Cairus's default operator.  If the source is semi-transparent, the over operation will
/// blend the source and the destination.  If the source is opaque, it will cover the destination
/// without blending.
fn over(source: &Rgba, destination: &mut Rgba) {
    let alpha = over_alpha(&source.alpha, &destination.alpha);
    let (red, green, blue) = (
        over_color(&source.red, &destination.red, &source.alpha, &destination.alpha, &alpha),
        over_color(&source.green, &destination.green, &source.alpha, &destination.alpha, &alpha),
        over_color(&source.blue, &destination.blue, &source.alpha, &destination.alpha, &alpha),
    );

    destination.red = red;
    destination.green = green;
    destination.blue = blue;
    destination.alpha = alpha;
}

fn over_color(source_color: &f32, destination_color: &f32,
              source_alpha: &f32, destination_alpha: &f32,
              new_alpha: &f32) -> f32 {
    (
        (source_color * source_alpha) +
        (destination_color * destination_alpha * (1. - source_alpha))
    ) / new_alpha
}

fn over_alpha(source: &f32, destination: &f32) -> f32 {
    source + (destination * (1. - source))
}

#[cfg(test)]
mod tests {
    use super::Operator;
    use super::over;
    use super::fetch_operator;
    use types::Rgba;

    #[test]
    fn test_over_operator_semi_transparent_source() {
        let source = Rgba::new(1., 0., 0., 0.5);
        let mut destination = Rgba::new(0., 1., 0., 0.5);
        over(&source, &mut destination);

        // This result was computed manually to be correct, and then modified to match Rust's
        // default floating point decimal place rounding.
        assert_eq!(destination, Rgba::new(0.6666667, 0.33333334, 0.0, 0.75));
    }

    #[test]
    fn test_over_operator_opaque_source() {
        let source = Rgba::new(1., 0., 0., 1.0);
        let mut destination = Rgba::new(0., 1., 1., 0.5);
        over(&source, &mut destination);
        assert_eq!(destination, Rgba::new(1., 0., 0., 1.0));
    }

    #[test]
    fn test_over_operator_opaque_destination() {
        let source = Rgba::new(0., 0., 1., 0.5);
        let mut destination = Rgba::new(0., 1., 0., 1.);
        over(&source, &mut destination);
        assert_eq!(destination, Rgba::new(0., 0.5, 0.5, 1.0));
    }

    #[test]
    fn test_fetch_operator() {
        let source = Rgba::new(1., 0., 0., 0.5);
        let mut destination = Rgba::new(0., 1., 0., 0.5);

        let myop = Operator::Over;
        let operator = fetch_operator(&myop);
        operator(&source, &mut destination);

        // This result was computed manually to be correct, and then modified to match Rust's
        // default floating point decimal place rounding.
        assert_eq!(destination, Rgba::new(0.6666667, 0.33333334, 0.0, 0.75));
    }
}
