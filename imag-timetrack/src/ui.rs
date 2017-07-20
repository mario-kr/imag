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

use clap::{Arg, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(SubCommand::with_name("start")
                   .about("Start time tracking")
                   .version("0.1")
                   .arg(Arg::with_name("start-time")
                        .index(1)
                        .required(true)
                        .help("Start-time when to start the timetracking (use 'now' for current time)"))
                   .arg(Arg::with_name("tags")
                        .index(2)
                        .required(true)
                        .multiple(true)
                        .help("Tags to start"))
                   )

       .subcommand(SubCommand::with_name("stop")
                   .about("Stop time tracking")
                   .version("0.1")
                   .arg(Arg::with_name("end-time")
                        .index(1)
                        .required(true)
                        .help("End-time when to stop the timetracking (use 'now' for current time)"))
                   .arg(Arg::with_name("tags")
                        .index(2)
                        .required(true)
                        .multiple(true)
                        .help("Tags to stop"))
                   )

       .subcommand(SubCommand::with_name("track")
                   .about("Track time in given range")
                   .version("0.1")
                   .arg(Arg::with_name("start-time")
                        .index(1)
                        .required(true)
                        .help("Start-time when to start the timetracking"))
                   .arg(Arg::with_name("end-time")
                        .index(2)
                        .required(true)
                        .help("End-time when to stop the timetracking"))
                   .arg(Arg::with_name("tags")
                        .index(3)
                        .required(true)
                        .multiple(true)
                        .help("Tags to stop"))
                   )

       .subcommand(SubCommand::with_name("continue")
                   .about("Continue last stopped time tracking")
                   .version("0.1")
                   )

       .subcommand(SubCommand::with_name("day")
                   .about("Print stats about day")
                   .version("0.1")
                   .arg(Arg::with_name("start")
                        .index(1)
                        .required(false)
                        .help("Limit to specific date and time, start time (default: today, 00:00:00)"))
                   .arg(Arg::with_name("end")
                        .index(2)
                        .required(false)
                        .help("Limit to specific date and time, end time (default: today, 23:59:59)"))
                   .arg(Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .required(false)
                        .multiple(true)
                        .help("Limit to certain tags"))
                   )

       .subcommand(SubCommand::with_name("week")
                   .about("Print stats about week")
                   .version("0.1")
                   .arg(Arg::with_name("start")
                        .index(1)
                        .required(false)
                        .help("Limit to specific date and time, start time (default: today, 00:00:00)"))
                   .arg(Arg::with_name("end")
                        .index(2)
                        .required(false)
                        .help("Limit to specific date and time, end time (default: today, 23:59:59)"))
                   .arg(Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .required(false)
                        .multiple(true)
                        .help("Limit to certain tags"))
                   )

       .subcommand(SubCommand::with_name("month")
                   .about("Print stats about month")
                   .version("0.1")
                   .arg(Arg::with_name("start")
                        .index(1)
                        .required(false)
                        .help("Limit to specific date and time, start time (default: today, 00:00:00)"))
                   .arg(Arg::with_name("end")
                        .index(2)
                        .required(false)
                        .help("Limit to specific date and time, end time (default: today, 23:59:59)"))
                   .arg(Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .required(false)
                        .multiple(true)
                        .help("Limit to certain tags"))
                   )

       .subcommand(SubCommand::with_name("year")
                   .about("Print stats about year")
                   .version("0.1")
                   .arg(Arg::with_name("start")
                        .index(1)
                        .required(false)
                        .help("Limit to specific date and time, start time (default: today, 00:00:00)"))
                   .arg(Arg::with_name("end")
                        .index(2)
                        .required(false)
                        .help("Limit to specific date and time, end time (default: today, 23:59:59)"))
                   .arg(Arg::with_name("tags")
                        .long("tags")
                        .short("t")
                        .required(false)
                        .multiple(true)
                        .help("Limit to certain tags"))
                   )

}
