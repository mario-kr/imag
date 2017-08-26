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

use regex::Regex;

pub type Tag = String;
pub type TagSlice<'a> = &'a str;

/// validator which can be used by clap to validate that a string is a valid tag
pub fn is_tag(s: String) -> Result<(), String> {
    is_tag_str(&s)
}

pub fn is_tag_str(s: &String) -> Result<(), String> {
    use filters::filter::Filter;

    let is_lower      = |s: &String| s.chars().all(|c| c.is_lowercase());
    let no_whitespace = |s: &String| s.chars().all(|c| !c.is_whitespace());
    let is_alphanum   = |s: &String| s.chars().all(|c| c.is_alphanumeric());
    let matches_regex = |s: &String| Regex::new("^[a-zA-Z]([a-zA-Z0-9_-]*)$").unwrap().captures(s).is_some();

    if is_lower.and(no_whitespace).and(is_alphanum).and(matches_regex).filter(s) {
        Ok(())
    } else {
        Err(format!("The string '{}' is not a valid tag", s))
    }
}

