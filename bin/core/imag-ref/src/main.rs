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

#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate clap;

extern crate libimagrt;
extern crate libimagentryref;
extern crate libimagerror;
extern crate libimagentrylist;
extern crate libimaginteraction;
extern crate libimagutil;

mod ui;
use ui::build_ui;

use std::path::PathBuf;
use std::process::exit;

use libimagentryref::refstore::RefStore;
use libimagentryref::flags::RefFlags;
use libimagerror::trace::trace_error;
use libimagerror::trace::MapErrTrace;
use libimagrt::setup::generate_runtime_setup;
use libimagrt::runtime::Runtime;

fn main() {
    let rt = generate_runtime_setup("imag-ref",
                                    &version!()[..],
                                    "Reference files outside of the store",
                                    build_ui);
    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call: {}", name);
            match name {
                "add"    => add(&rt),
                "remove" => remove(&rt),
                "list"   => list(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            };
        });
}

fn add(rt: &Runtime) {
    let cmd  = rt.cli().subcommand_matches("add").unwrap();
    let path = cmd.value_of("path").map(PathBuf::from).unwrap(); // saved by clap

    let flags = RefFlags::default()
        .with_content_hashing(cmd.is_present("track-content"))
        .with_permission_tracking(cmd.is_present("track-permissions"));

    match RefStore::create(rt.store(), path, flags) {
        Ok(r) => {
            debug!("Reference created: {:?}", r);
            info!("Ok");
        },
        Err(e) => {
            trace_error(&e);
            warn!("Failed to create reference");
        },
    }
}

fn remove(rt: &Runtime) {
    use libimaginteraction::ask::ask_bool;

    let cmd  = rt.cli().subcommand_matches("remove").unwrap();
    let hash = cmd.value_of("hash").map(String::from).unwrap(); // saved by clap
    let yes  = cmd.is_present("yes");

    match rt.store().find_storeid_by_partial_hash(&hash).map_err_trace_exit_unwrap(1) {
        Some(sid) => {
            if yes || ask_bool(&format!("Delete Ref with hash '{}'", hash)[..], None) {
                debug!("Found for hash '{}' -> {:?}", hash, sid);
                rt.store().delete(sid).map_err_trace_exit_unwrap(1)
            } else {
                info!("Aborted");
            }
        },
        None => {
            error!("Not id for hash '{}' found", hash);
            exit(1)
        },
    };

}

fn list(rt: &Runtime) {
    use std::process::exit;

    use libimagentrylist::lister::Lister;
    use libimagentryref::lister::RefLister;

    let cmd                      = rt.cli().subcommand_matches("list").unwrap();
    let do_check_dead            = cmd.is_present("check-dead");
    let do_check_changed         = cmd.is_present("check-changed");
    let do_check_changed_content = cmd.is_present("check-changed-content");
    let do_check_changed_permiss = cmd.is_present("check-changed-permissions");

    let iter = match rt.store().retrieve_for_module("ref") {
        Ok(iter) => iter.filter_map(|id| {
            match rt.store().get(id) {
                Ok(r) => Some(r),
                Err(e) => {
                    trace_error(&e);
                    None
                },
            }
        }),
        Err(e) => {
            trace_error(&e);
            exit(1);
        }
    };

    RefLister::new()
        .check_dead(do_check_dead)
        .check_changed(do_check_changed)
        .check_changed_content(do_check_changed_content)
        .check_changed_permiss(do_check_changed_permiss)
        .list(iter.filter_map(Into::into))
        .ok();
}

