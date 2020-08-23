mod encode;
mod decode;

use clap::{crate_version, crate_authors, App, Arg};

fn main() {
    let matches = App::new("urlsafe")
        .about("encodes and decodes de-fanged URLs.")
    // use crate_version! to pull the version number
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .value_name("URL")
                .help("De-fangs a malicious URL.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .value_name("URL")
                .help("Returns a malicious URL to its dangerous state.")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    match matches.value_of("encode") {
        Some(e) => println!("{}", encode::url(String::from(e))),
        None => (),
    };

    match matches.value_of("decode") {
        Some(d) => println!("{}", decode::url(String::from(d))),
        None => (),
    };
}
