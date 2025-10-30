// Spectre Password Manager - Rust Implementation
// 
// This is a derivative work based on the original Spectre algorithm by Maarten Billemont.
// Original: https://spectre.app
// Original Copyright: © 2011-2017 Maarten Billemont
// Original License: GNU General Public License v3.0
//
// This Rust Implementation:
// Copyright (c) 2025 Abdulrhman Alkhodiry
// Licensed under GNU General Public License v3.0 or later
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

pub mod algorithm;
pub mod error;
pub mod models;
pub mod marshal;
pub mod types;
pub mod util;

// Re-export commonly used items
pub use algorithm::{
    spectre_user_key, 
    spectre_site_result, 
    spectre_identicon,
    spectre_identicon_render,
    spectre_site_state,
    SpectreUserKey,
};
pub use error::{SpectreError, Result};
pub use models::*;
pub use marshal::{spectre_marshal_read, spectre_marshal_write, spectre_marshal_auth, spectre_user_path};
pub use types::*;
pub use util::*;
