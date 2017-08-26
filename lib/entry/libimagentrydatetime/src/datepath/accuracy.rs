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

/// The accuracy with which the compiler should compile the time specification
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Accuracy {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second
}

impl Accuracy {

    /// Check whether the current setting includes a year.
    pub fn has_year_accuracy(&self) -> bool {
        match *self {
            Accuracy::Year   => true,
            Accuracy::Month  => true,
            Accuracy::Day    => true,
            Accuracy::Hour   => true,
            Accuracy::Minute => true,
            Accuracy::Second => true,
        }
    }

    /// Check whether the current setting includes a month.
    pub fn has_month_accuracy(&self) -> bool {
        match *self {
            Accuracy::Year   => false,
            Accuracy::Month  => true,
            Accuracy::Day    => true,
            Accuracy::Hour   => true,
            Accuracy::Minute => true,
            Accuracy::Second => true,
        }
    }

    /// Check whether the current setting includes a day.
    pub fn has_day_accuracy(&self) -> bool {
        match *self {
            Accuracy::Year   => false,
            Accuracy::Month  => false,
            Accuracy::Day    => true,
            Accuracy::Hour   => true,
            Accuracy::Minute => true,
            Accuracy::Second => true,
        }
    }

    /// Check whether the current setting includes a hour.
    pub fn has_hour_accuracy(&self) -> bool {
        match *self {
            Accuracy::Year   => false,
            Accuracy::Month  => false,
            Accuracy::Day    => false,
            Accuracy::Hour   => true,
            Accuracy::Minute => true,
            Accuracy::Second => true,
        }
    }

    /// Check whether the current setting includes a minute.
    pub fn has_minute_accuracy(&self) -> bool {
        match *self {
            Accuracy::Year   => false,
            Accuracy::Month  => false,
            Accuracy::Day    => false,
            Accuracy::Hour   => false,
            Accuracy::Minute => true,
            Accuracy::Second => true,
        }
    }

    /// Check whether the current setting includes a second.
    pub fn has_second_accuracy(&self) -> bool {
        match *self {
            Accuracy::Year   => false,
            Accuracy::Month  => false,
            Accuracy::Day    => false,
            Accuracy::Hour   => false,
            Accuracy::Minute => false,
            Accuracy::Second => true,
        }
    }

}

impl Default for Accuracy {
    fn default() -> Accuracy {
        Accuracy::Second
    }
}

