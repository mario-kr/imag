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

use toml_query::insert::TomlValueInsertExt;
use toml_query::read::TomlValueReadExt;
use toml_query::error::ErrorKind as TQEK;
use toml::Value;

use libimagstore::store::Entry;
use libimagerror::into::IntoError;

use error::CategoryErrorKind as CEK;
use error::MapErrInto;
use result::Result;
use register::CategoryRegister;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Category(String);

impl From<String> for Category {

    fn from(s: String) -> Category {
        Category(s)
    }

}

impl Into<String> for Category {
    fn into(self) -> String {
        self.0
    }
}

pub trait EntryCategory {

    fn set_category(&mut self, s: Category) -> Result<()>;

    fn set_category_checked(&mut self, register: &CategoryRegister, s: Category) -> Result<()>;

    fn get_category(&self) -> Result<Option<Category>>;

    fn has_category(&self) -> Result<bool>;

}

impl EntryCategory for Entry {

    fn set_category(&mut self, s: Category) -> Result<()> {
        self.get_header_mut()
            .insert(&String::from("category.value"), Value::String(s.into()))
            .map_err_into(CEK::HeaderWriteError)
            .map(|_| ())
    }

    /// Check whether a category exists before setting it.
    ///
    /// This function should be used by default over EntryCategory::set_category()!
    fn set_category_checked(&mut self, register: &CategoryRegister, s: Category) -> Result<()> {
        register.category_exists(&s.0)
            .and_then(|bl| if bl {
                self.set_category(s)
            } else {
                Err(CEK::CategoryDoesNotExist.into_error())
            })
    }

    fn get_category(&self) -> Result<Option<Category>> {
        match self.get_header().read(&String::from("category.value")) {
            Err(res) => match res.kind() {
                &TQEK::IdentifierNotFoundInDocument(_) => Ok(None),
                _                                      => Err(res),
            }
            .map_err_into(CEK::HeaderReadError),

            Ok(Some(&Value::String(ref s))) => Ok(Some(s.clone().into())),
            Ok(None) => Err(CEK::StoreReadError.into_error()).map_err_into(CEK::HeaderReadError),
            Ok(_) => Err(CEK::TypeError.into_error()).map_err_into(CEK::HeaderReadError),
        }
    }

    fn has_category(&self) -> Result<bool> {
        self.get_header().read(&String::from("category.value"))
            .map_err_into(CEK::HeaderReadError)
            .map(|e| e.is_some())
    }

}
