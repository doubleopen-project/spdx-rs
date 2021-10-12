// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::non_ascii_literal
)]

pub mod error;
mod graph;
pub mod license_list;
pub mod models;
pub mod parsers;
