mod rgrokCLI;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("rgrok")
        .version("1.0")
        .author("Jonathan Borg <jonathan@x3m.io>")
        .about("Create a tunnel to your local dev server")
        .before_help("")
        .arg(
            Arg::with_name("hostname")
                .short("h")
                .long("hostname")
                .value_name("HOSTNAME")
                .help("The host to your server that is running rgrokd")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("httpauth")
                .short("a")
                .long("httpauth")
                .value_name("HTTPAUTH")
                .help("user:password")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("PORT")
                .help("Port of your local dev server")
                .required(true)
                .index(1),
        )
        .get_matches();

    // // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    let hostname = String::from(matches.value_of("HOSTNAME").unwrap());
    let httpauth = String::from(matches.value_of("HTTPAUTH").unwrap());
    rgrokCLI::connect(hostname, httpauth);
    println!("{:?}", matches)
}
