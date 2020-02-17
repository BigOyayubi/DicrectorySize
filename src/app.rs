use clap::{self, App, AppSettings, Arg, crate_authors, crate_version};

const ABOUT: &str = "
dirsize (ds) calculates your set directory for a regex pattern.
";

const USAGE: &str = "
    ds [OPTIONS] [DIR]
    ds [OPTIONS] [-i PATTERN] [-e PATTERN] [DIR]
";

const TEMPLATE: &str = "\
{bin} {version}
{author}
{about}

USAGE:{usage}

ARGS:
{positionals}

OPTIONS:
{unified}";

pub fn app() -> App<'static, 'static> {
    let app = App::new("dirsize")
        .author(crate_authors!())
        .version(crate_version!())
        .about(ABOUT)
        .max_term_width(100)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::AllArgsOverrideSelf)
        .usage(USAGE)
        .template(TEMPLATE)
        .help_message("Prints help information. Use --help for more details.")
        .arg(Arg::with_name("path").help("calculate root directory path").required(true))
        .arg(Arg::with_name("kilo").help("show size in kilobytes").short("k").long("kilo"))
        .arg(Arg::with_name("depth").help("digg dir depth").short("d").long("depth").takes_value(true))
        .arg(Arg::with_name("include").help("include search pattern").short("i").long("include").takes_value(true))
        .arg(Arg::with_name("exclude").help("exclude search pattern").short("e").long("exclude").takes_value(true))
        .arg(Arg::with_name("debug").help("show debug messages").long("debug"))
        .arg(Arg::with_name("trace").help("show trace log").long("trace"));
    app
}

