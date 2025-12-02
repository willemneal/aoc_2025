use std::{env, fs, path::Path, str::{FromStr}};

pub fn parse_input_file_lines_from_args<T: FromStr>(separator: &str) -> Result<Vec<T>, T::Err> {
    let mut args = env::args();
    args.next();
    parse_file_lines(&args.next().unwrap(), separator)
}

pub fn parse_file_lines<P: AsRef<Path>, T: FromStr>(path: P, separator: &str) -> Result<Vec<T>, T::Err> {
    let input = fs::read_to_string(&path).unwrap();
    input
        .split(separator)
        .map(FromStr::from_str)
        .collect::<Result<Vec<T>, T::Err>>()
}
