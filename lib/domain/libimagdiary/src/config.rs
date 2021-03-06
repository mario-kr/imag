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

use toml::Value;

use libimagrt::runtime::Runtime;

use toml_query::read::TomlValueReadExt;

pub fn get_default_diary_name(rt: &Runtime) -> Option<String> {
    get_diary_config_section(rt)
        .and_then(|config| {
            match config.read(&String::from("default_diary")) {
                Ok(Some(&Value::String(ref s))) => Some(s.clone()),
                _ => None,
            }
        })
}

pub fn get_diary_config_section<'a>(rt: &'a Runtime) -> Option<&'a Value> {
    rt.config().and_then(|config| match config.read(&String::from("diary")) {
        Ok(x)  => x,
        Err(_) => None,
    })
}
