use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.0.1")
        .author("Thiago")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .conflicts_with("bytes")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .value_name("bytes")
                .help("Show line count")
                .takes_value(false)
        )
    .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    if [lines,words,bytes,chars].iter().all(|v| v== &false) {
        lines=true;
        words=true;
        bytes=true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines,
        words: words,
        bytes: bytes,
        chars: chars
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}