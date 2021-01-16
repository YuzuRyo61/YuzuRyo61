//! # YuzuRyo61
//! Introduction program of YuzuRyo61 profile.

use yuzuryo61::variables::{BLOG_URL, SITE_URL, get_clap_app};

/// メインプログラム
fn main() -> std::io::Result<()> {
    let app = get_clap_app();
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
