// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Library for Advent of Code solutions.
//!
//! This library contains support routines for the Advent of
//! Code Rust solutions. An attempt has been made to put
//! common code here rather than copy-pasting. In addition,
//! material of possible general use has been put here rather
//! than burying it in a solution.

// For an explanation of the structure of this file, see
// http://stackoverflow.com/questions/22596920/

pub mod args;
pub use self::args::*;

pub mod lines;
pub use self::lines::*;

pub mod trace;
pub use self::trace::*;

pub mod into_chars;
pub use self::into_chars::*;

#[cfg(feature = "astar")]
pub extern crate astar;
#[cfg(feature = "comb")]
pub extern crate comb;
#[cfg(feature = "geom")]
pub extern crate geom;
#[cfg(feature = "hexstring")]
pub extern crate hexstring;
#[cfg(feature = "maprender")]
pub extern crate maprender;
#[cfg(feature = "numberfns")]
pub extern crate numberfns;
#[cfg(feature = "reparse")]
pub extern crate reparse;
