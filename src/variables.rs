use clap::{SubCommand, AppSettings, Arg, App};

pub const SITE_URL: &str = "https://yuzuryo61.me/";
pub const BLOG_URL: &str = "https://yuzulia.com/";

pub fn get_clap_app<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!()
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
        )
}
