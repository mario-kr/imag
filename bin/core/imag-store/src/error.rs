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

use std::error::Error;

use libimagerror::into::IntoError;

error_chain! {
    types {
        StoreError, StoreErrorKind, ResultExt, Result;
    }

    errors {
        BackendError      {
            description("Backend Error")
            display("Backend Error")
        }

        NoCommandlineCall {
            description("No commandline call")
            display("No commandline call")
        }

    }
}

impl IntoError for StoreErrorKind {
    type Target = StoreError;

    fn into_error(self) -> Self::Target {
        StoreError::from_kind(self)
    }

    fn into_error_with_cause(self, _: Box<Error>) -> Self::Target {
        StoreError::from_kind(self)
    }
}
