use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.0.1")
        .author("Thiago")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(true)
                .conflicts_with("number_bytes"),
        )
        .arg(
            Arg::with_name("number bytes")
                .short("b")
                .long("number_bytes")
                .help("Number bytes")
                .takes_value(true)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config { 
        files: matches.values_of_lossy("files").unwrap(), 
        lines: matches.value_of("number").unwrap().parse().unwrap(), 
        bytes: Some(0)//matches.value_of("number_bytes").unwrap().parse().unwrap(), 
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}",config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n>0 => Ok(n),
        _ => Err(From::from(val)),
    }
    
}

#[test]
fn test_parse_positive_int() {
    let res=parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),3);

    //string=err
    let res=parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),"foo".to_string());

    //0=error
    let res=parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),"0".to_string());
}