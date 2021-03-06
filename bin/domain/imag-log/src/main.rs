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

#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate clap;
#[macro_use] extern crate is_match;
#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate toml;
extern crate toml_query;

extern crate libimaglog;
extern crate libimagrt;
extern crate libimagerror;
extern crate libimagdiary;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;
use libimagdiary::diary::Diary;
use libimaglog::log::Log;
use libimaglog::error::LogError as LE;

mod ui;
use ui::build_ui;

use toml::Value;

fn main() {
    let rt = generate_runtime_setup("imag-log",
                                    &version!()[..],
                                    "Overlay to imag-diary to 'log' single lines of text",
                                    build_ui);


    if let Some(scmd) = rt.cli() .subcommand_name() {
        match scmd {
            "show" => show(&rt),
            _        => {
                error!("Unknown command");
                ::std::process::exit(1)
            },
        }
    } else {
        let text       = get_log_text(&rt);
        let diary_name = rt.cli()
            .value_of("diaryname")
            .map(String::from)
            .unwrap_or_else(|| get_diary_name(&rt));

        debug!("Writing to '{}': {}", diary_name, text);

        let _ = rt
            .store()
            .new_entry_now(&diary_name)
            .map(|mut fle| {
                let _ = fle.make_log_entry().map_err_trace_exit_unwrap(1);
                *fle.get_content_mut() = text;
            })
            .map_err_trace_exit_unwrap(1);
        info!("Ok");
    }
}

fn show(rt: &Runtime) {
    use libimagdiary::iter::DiaryEntryIterator;
    use libimagdiary::entry::DiaryEntry;

    let scmd = rt.cli().subcommand_matches("show").unwrap(); // safed by main()
    let iters : Vec<DiaryEntryIterator> = match scmd.values_of("show-name") {
        Some(values) => values
            .map(|diary_name| Diary::entries(rt.store(), diary_name).map_err_trace_exit_unwrap(1))
            .collect(),

        None => if scmd.is_present("show-all") {
            rt.store()
                .diary_names()
                .map_err_trace_exit_unwrap(1)
                .map(|diary_name| {
                    let diary_name = diary_name.map_err_trace_exit_unwrap(1);
                    Diary::entries(rt.store(), &diary_name).map_err_trace_exit_unwrap(1)
                })
                .collect()
        } else {
            // showing default logs
            vec![Diary::entries(rt.store(), &get_diary_name(rt)).map_err_trace_exit_unwrap(1)]
        }
    };

    for iter in iters {
        for element in iter {
            let e  = element.map_err_trace_exit_unwrap(1);
            let id = e.diary_id().map_err_trace_exit_unwrap(1);
            println!("{dname: >10} - {y: >4}-{m:0>2}-{d:0>2}T{H:0>2}:{M:0>2} - {text}",
                     dname = id.diary_name(),
                     y = id.year(),
                     m = id.month(),
                     d = id.day(),
                     H = id.hour(),
                     M = id.minute(),
                     text = e.get_content());
        }
    }

    info!("Ready.");
}

fn get_diary_name(rt: &Runtime) -> String {
    use toml_query::read::TomlValueReadExt;

    let cfg = rt
        .config()
        .ok_or(LE::from("Configuration not present, cannot continue"))
        .map_err_trace_exit_unwrap(1);

    let logs = cfg
        .read("log.logs")
        .map_err_trace_exit_unwrap(1)
        .ok_or(LE::from("Configuration missing: 'log.logs'"))
        .map_err_trace_exit_unwrap(1)
        .as_array()
        .ok_or(LE::from("Configuration 'log.logs' is not an Array"))
        .map_err_trace_exit_unwrap(1);

    if !logs.iter().all(|e| is_match!(e, &Value::String(_))) {
        error!("Configuration 'log.logs' is not an Array<String>!");
        ::std::process::exit(1);
    }

    let logs = logs
        .into_iter()
        .map(Value::as_str)
        .map(Option::unwrap)
        .collect::<Vec<_>>();

    let current_log = cfg
        .read("log.default")
        .map_err_trace_exit_unwrap(1)
        .ok_or(LE::from("Configuration missing: 'log.default'"))
        .map_err_trace_exit_unwrap(1)
        .as_str()
        .ok_or(LE::from("Configuration 'log.default' is not a String"))
        .map_err_trace_exit_unwrap(1);

    if !logs.contains(&current_log) {
        error!("'log.logs' does not contain 'log.default'");
        ::std::process::exit(1)
    } else {
        current_log.into()
    }

}

fn get_log_text(rt: &Runtime) -> String {
    rt.cli()
        .values_of("text")
        .unwrap() // safe by clap
        .fold(String::with_capacity(500), |mut acc, e| {
            acc.push_str(" ");
            acc.push_str(e);
            acc
        })
}
