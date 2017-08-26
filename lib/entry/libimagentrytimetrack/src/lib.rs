//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

extern crate filters;
extern crate chrono;
extern crate toml;
extern crate toml_query;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate is_match;

#[macro_use]
extern crate libimagerror;
#[macro_use]
extern crate libimagstore;
extern crate libimagentrydatetime;
extern crate libimagentrytag;

mod constants;
pub mod error;
pub mod timetracking;
pub mod timetrackingstore;
pub mod iter;
pub mod result;
pub mod tag;

module_entry_path_mod!("timetrack");

