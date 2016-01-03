use std::fmt::{Debug, Formatter};
use std::fmt::Result as FMTResult;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use clap::ArgMatches;
use regex::Regex;

mod header;

use module::Module;
use runtime::Runtime;
use storage::file::File;
use storage::parser::Parser;
use storage::json::parser::JsonHeaderParser;
use module::helpers::cli::create_tag_filter;
use module::helpers::cli::create_hash_filter;
use module::helpers::cli::create_text_header_field_grep_filter;
use module::helpers::cli::create_content_grep_filter;
use module::helpers::cli::CliFileFilter;

pub struct Notes<'a> {
    rt: &'a Runtime<'a>,
}

impl<'a> Notes<'a> {

    pub fn new(rt: &'a Runtime<'a>) -> Notes<'a> {
        Notes {
            rt: rt,
        }
    }

    fn command_add(&self, matches: &ArgMatches) -> bool {
        use std::process::exit;
        use self::header::build_header;
        use ui::external::editor::let_user_provide_content;

        let parser = Parser::new(JsonHeaderParser::new(None));
        let name   = matches.value_of("name")
                            .map(String::from)
                            .unwrap_or(String::from(""));
        let tags   = matches.value_of("tags")
                            .and_then(|s| Some(s.split(",").map(String::from).collect()))
                            .unwrap_or(vec![]);

        debug!("Building header with");
        debug!("    name = '{:?}'", name);
        debug!("    tags = '{:?}'", tags);
        let header = build_header(name, tags);

        let content = let_user_provide_content(self.runtime()).unwrap_or(String::from(""));

        let fileid = self.rt.store().new_file_with_content(self, header, content);
        self.rt
            .store()
            .load(self, &parser, &fileid)
            .and_then(|file| {
                info!("Created file in memory: {}", fileid);
                Some(self.rt.store().persist(&parser, file))
            })
            .unwrap_or(false)
    }

    fn command_edit(&self, matches: &ArgMatches) -> bool {
        use ui::external::editor::edit_content;

        let parser  = Parser::new(JsonHeaderParser::new(None));

        let filter = {
            let hash_filter = create_hash_filter(matches, "id", false);
            let head_filter = create_text_header_field_grep_filter(matches, "namematch", "NAME", false);
            let text_filter = create_content_grep_filter(matches, "match", false);
            let tags_filter = create_tag_filter(matches, "tags", false);
            hash_filter.or(Box::new(head_filter)).or(Box::new(text_filter)).or(Box::new(tags_filter))
        };

        let result = self.rt
            .store()
            .load_for_module(self, &parser)
            .into_iter()
            .filter(|f| filter.filter_file(f))
            .map(|file| {
                debug!("File loaded, can edit now: {:?}", file);

                let old_content = {
                    let f = file.deref().borrow();
                    f.data().clone()
                };

                debug!("Editing content now...");
                let (new_content, editing_worked) = edit_content(self.runtime(), old_content);
                debug!("... ready with editing");

                if editing_worked {
                    debug!("Editing worked");
                    {
                        let mut f = file.deref().borrow_mut();
                        f.set_data(new_content);
                    }
                    self.runtime().store().persist(&parser, file)
                } else {
                    debug!("Editing didn't work");
                    false
                }
            })
            .fold((0, 0), |acc, succeeded| {
                let (worked, failed) = acc;
                if succeeded {
                    (worked + 1, failed)
                } else {
                    (worked, failed + 1)
                }
            });

        let (worked, failed) = result;

        info!("Editing succeeded for {} files", worked);
        info!("Editing failed for {} files", failed);

        return failed == 0;
    }

    fn command_list(&self, matches: &ArgMatches) -> bool {
        use ui::file::{FilePrinter, TablePrinter};
        use self::header::get_name_from_header;
        use self::header::get_tags_from_header;
        use std::process::exit;
        use module::helpers::cli::CliFileFilter;

        let parser  = Parser::new(JsonHeaderParser::new(None));

        let filter = {
            let hash_filter = create_hash_filter(matches, "id", true);
            let head_filter = create_text_header_field_grep_filter(matches, "match", "NAME", true);
            let text_filter = create_content_grep_filter(matches, "match", true);
            let tags_filter = create_tag_filter(matches, "tags", true);
            hash_filter.or(Box::new(head_filter)).and(Box::new(text_filter)).and(Box::new(tags_filter))
        };

        let printer = TablePrinter::new(self.rt.is_verbose(), self.rt.is_debugging());

        printer.print_files_custom(
            self.rt.store()
                    .load_for_module(self, &parser)
                    .into_iter()
                    .filter(|f| filter.filter_file(f)),
            &|file| {
                let fl      = file.deref().borrow();
                let hdr     = fl.header();
                let name    = get_name_from_header(hdr);
                let tags    = get_tags_from_header(hdr);

                debug!("Custom printer field: name = '{:?}'", name);
                debug!("Custom printer field: tags = '{:?}'", tags);

                vec![name, tags.join(", ")]
            }
        );
        true
    }

    fn command_links(&self, matches: &ArgMatches) -> bool {
        use module::helpers::content::markdown::MarkdownParser;
        use ui::file::{FilePrinter, TablePrinter};
        use util::is_url;
        use prettytable::Table;
        use prettytable::row::Row;
        use prettytable::cell::Cell;
        use itertools::Itertools;

        debug!("Going to list links in files...");

        let list_intern = matches.is_present("internal");
        let list_extern = matches.is_present("external");
        debug!("list internal links = {}", list_intern);
        debug!("list external links = {}", list_extern);

        let printer = TablePrinter::new(self.rt.is_verbose(), self.rt.is_debugging());
        let titles = row!["#", "Text", "Link", "Direction"];
        let mut table = Table::new();
        table.set_titles(titles);
        debug!("Table setup finished");

        let parser = Parser::new(JsonHeaderParser::new(None));
        let filter = {
            let hash_filter = create_hash_filter(matches, "id", false);
            let text_filter = create_text_header_field_grep_filter(matches, "match", "URL", false);
            let tags_filter = create_tag_filter(matches, "tags", false);
            hash_filter.or(Box::new(text_filter)).or(Box::new(tags_filter))
        };

        let result = self.rt
            .store()
            .load_for_module(self, &parser)
            .iter()
            .filter(|file| {
                let res = filter.filter_file(file);
                debug!("Filter: {} -> {}", file.deref().borrow().id(), res);
                res
            })
            .map(|file| {
                debug!("File loaded, can parse for links now: {}", file.deref().borrow().id());
                let data = {
                    let f = file.deref().borrow();
                    debug!("Parsing markdown in file = {:?}", f);
                    f.data().clone()
                };
                let links = MarkdownParser::new(&data).links();
                debug!("Retreived {} links from {}", links.len(), file.deref().borrow().id());
                links
            })
            .flatten()
            .filter(|link| {
                let title       = &link.title;
                let url         = &link.url;
                let is_extern   = is_url(&url);
                debug!("Is external URL {} -> {}", url, is_extern);
                debug!("List external URLs -> {}", list_extern);
                debug!("List internal URLs -> {}", list_intern);
                ((!list_intern && !list_extern) ||
                 (is_extern && list_extern)     ||
                 (!is_extern && list_intern))
            })
            .enumerate()
            .map(|(i_link, link)| {
                let title   = &link.title;
                let url     = &link.url;
                let is_url  = is_url(&url);
                debug!("Listing: {} -> {}", title, url);

                let linkno_cell = Cell::new(&format!("{}", i_link)[..]);
                let title_cell  = Cell::new(&format!("{}", title)[..]);
                let url_cell    = Cell::new(&format!("{}", url)[..]);
                let dir_cell    = Cell::new(if is_url { "extern" } else { "intern" });

                let r = Row::new(vec![linkno_cell,
                                      title_cell,
                                      url_cell,
                                      dir_cell]);
                table.add_row(r);
                true
            })
            .fold((0, 0), |acc, succeeded| {
                let (worked, failed) = acc;
                if succeeded {
                    (worked + 1, failed)
                } else {
                    (worked, failed + 1)
                }
            });

        let (worked, failed) = result;

        if worked != 0 {
            debug!("Printing table entries");
            table.printstd();
        } else {
            debug!("Not printing table as there wouldn't be any entries in it");
        }

        info!("Listing links succeeded for {} files", worked);
        info!("Listing links failed for {} files", failed);

        return failed == 0;
    }

    fn command_remove(&self, matches: &ArgMatches) -> bool {
        let parser = Parser::new(JsonHeaderParser::new(None));

        let filter = {
            let hash_filter = create_hash_filter(matches, "id", false);
            let text_filter = create_text_header_field_grep_filter(matches, "match", "URL", false);
            let tags_filter = create_tag_filter(matches, "tags", false);
            hash_filter.or(Box::new(text_filter)).or(Box::new(tags_filter))
        };

        let result = self.rt
            .store()
            .load_for_module(self, &parser)
            .iter()
            .filter(|file| filter.filter_file(file))
            .map(|file| {
                debug!("File loaded, can remove now: {:?}", file);
                let f = file.deref().borrow();
                self.rt.store().remove(f.id().clone())
            })
            .fold((0, 0), |acc, succeeded| {
                let (worked, failed) = acc;
                if succeeded {
                    (worked + 1, failed)
                } else {
                    (worked, failed + 1)
                }
            });

        let (worked, failed) = result;

        info!("Removing succeeded for {} files", worked);
        info!("Removing failed for {} files", failed);

        return failed == 0;
    }

    fn command_add_tags(&self, matches: &ArgMatches) -> bool {
        use module::helpers::header::tags::data::alter_tags_in_files;
        use self::header::rebuild_header_with_tags;

        let parser = Parser::new(JsonHeaderParser::new(None));
        alter_tags_in_files(self, matches, &parser, |old_tags, cli_tags| {
            let mut new_tags = old_tags.clone();
            new_tags.append(&mut cli_tags.clone());
            new_tags
        }, rebuild_header_with_tags)
    }

    fn command_rm_tags(&self, matches: &ArgMatches) -> bool {
        use module::helpers::header::tags::data::alter_tags_in_files;
        use self::header::rebuild_header_with_tags;

        let parser = Parser::new(JsonHeaderParser::new(None));
        alter_tags_in_files(self, matches, &parser, |old_tags, cli_tags| {
            old_tags.clone()
                .into_iter()
                .filter(|tag| !cli_tags.contains(tag))
                .collect()
        }, rebuild_header_with_tags)
    }

    fn command_set_tags(&self, matches: &ArgMatches) -> bool {
        use module::helpers::header::tags::data::alter_tags_in_files;
        use self::header::rebuild_header_with_tags;

        let parser = Parser::new(JsonHeaderParser::new(None));
        alter_tags_in_files(self, matches, &parser, |old_tags, cli_tags| {
            cli_tags.clone()
        }, rebuild_header_with_tags)
    }

}

impl<'a> Module<'a> for Notes<'a> {

    fn exec(&self, matches: &ArgMatches) -> bool {
        match matches.subcommand_name() {
            Some("add") => {
                self.command_add(matches.subcommand_matches("add").unwrap())
            },

            Some("edit") => {
                self.command_edit(matches.subcommand_matches("edit").unwrap())
            },

            Some("list") => {
                self.command_list(matches.subcommand_matches("list").unwrap())
            },

            Some("links") => {
                self.command_links(matches.subcommand_matches("links").unwrap())
            },

            Some("remove") => {
                self.command_remove(matches.subcommand_matches("remove").unwrap())
            },

            Some("add_tags") => {
                self.command_add_tags(matches.subcommand_matches("add_tags").unwrap())
            },

            Some("rm_tags") => {
                self.command_rm_tags(matches.subcommand_matches("rm_tags").unwrap())
            },

            Some("set_tags") => {
                self.command_set_tags(matches.subcommand_matches("set_tags").unwrap())
            },

            Some(_) | None => {
                info!("No command given, doing nothing");
                false
            },
        }
    }

    fn name(&self) -> &'static str{
        "notes"
    }

    fn runtime(&self) -> &Runtime {
        self.rt
    }

}

impl<'a> Debug for Notes<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> FMTResult {
        write!(fmt, "[Module][Notes]");
        Ok(())
    }

}
