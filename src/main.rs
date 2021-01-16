//! # YuzuRyo61
//! Introduction program of YuzuRyo61 profile.

#[macro_use]
extern crate clap;

use clap::{SubCommand, AppSettings, Arg};
use yuzuryo61::variables::{BLOG_URL, SITE_URL};

/// メインプログラム
fn main() -> std::io::Result<()> {
    let app = app_from_crate!()
        .name("YuzuRyo61")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("site")
            .about("Open browser(or print URL) to my site")
            .arg(Arg::with_name("no_open")
                .short("o")
                .long("no-open")
                .help("Do not open the browser")
                .required(false)
                .takes_value(false))
        )
        .subcommand(SubCommand::with_name("blog")
            .about("Open browser(or print URL) to blog site")
            .arg(Arg::with_name("no_open")
                .short("o")
                .long("no-open")
                .help("Do not open the browser")
                .required(false)
                .takes_value(false))
        );
    let matches = app.get_matches();

    if let Some(am) = matches.subcommand_matches("site") {
        if !am.is_present("no_open") {
            if let Err(_) = webbrowser::open(&SITE_URL) {
                println!("{}", SITE_URL);
            }
        } else {
            println!("{}", SITE_URL);
        }
        return Ok(());
    };

    if let Some(am) = matches.subcommand_matches("blog") {
        if !am.is_present("no_open") {
            if let Err(_) = webbrowser::open(&BLOG_URL) {
                println!("{}", BLOG_URL)
            }
        } else {
            println!("{}", BLOG_URL);
        }
        return Ok(());
    };

    Ok(())
}
