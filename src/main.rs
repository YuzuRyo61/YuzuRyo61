#[macro_use]
extern crate clap;

use clap::{SubCommand, AppSettings, Arg};

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
        let site_url = "https://yuzuryo61.me";
        if !am.is_present("no_open") {
            if let Err(_) = webbrowser::open(&site_url) {
                println!("{}", site_url);
            }
        } else {
            println!("{}", site_url);
        }
        return Ok(());
    };

    if let Some(am) = matches.subcommand_matches("blog") {
        let site_url = "https://yuzulia.com/";
        if !am.is_present("no_open") {
            if let Err(_) = webbrowser::open(&site_url) {
                println!("{}", site_url)
            }
        } else {
            println!("{}", site_url);
        }
        return Ok(());
    };

    Ok(())
}
