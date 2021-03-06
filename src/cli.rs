use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, ArgMatches};

pub fn launch<'a>() -> ArgMatches<'a> {
    App::new("t-rec")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::AllowMissingPositional)
        .arg(
            Arg::with_name("list-windows")
                .value_name("list all visible windows with name and id")
                .takes_value(false)
                .required(false)
                .short("l")
                .long("ls-win")
                .help("If you want to see a list of windows available for recording by their id, you can set env var 'WINDOWID' to record this specific window only."),
        )
        .arg(
            Arg::with_name("program")
                .value_name("shell or program to launch")
                .takes_value(true)
                .required(false)
                .help("If you want to start a different program than $SHELL you can pass it here. For example '/bin/sh'"),
        ).get_matches()
}
