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

use libimagstore::store::Entry;

use toml_query::read::TomlValueReadExt;
use toml_query::set::TomlValueSetExt;

use error::Result;
use error::NoteErrorKind as NEK;
use error::NoteError as NE;
use error::ResultExt;

pub trait Note {
    fn set_name(&mut self, n: String) -> Result<()>;
    fn get_name(&self) -> Result<String>;
    fn set_text(&mut self, n: String);
    fn get_text(&self) -> &String;
}

impl Note for Entry {

    fn set_name(&mut self, n: String) -> Result<()> {
        self.get_header_mut()
            .set("note.name", Value::String(n))
            .chain_err(|| NEK::StoreWriteError)
            .map(|_| ())
    }

    fn get_name(&self) -> Result<String> {
        self.get_header()
            .read("note.name")
            .chain_err(|| NEK::StoreReadError)?
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or(NE::from_kind(NEK::HeaderTypeError))
    }

    fn set_text(&mut self, n: String) {
        *self.get_content_mut() = n
    }

    fn get_text(&self) -> &String {
        self.get_content()
    }

}


