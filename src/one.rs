// My Type

impl FromStr for MyType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn run(input: &[MyType]) {
    todo!()
}

fn main() {
    let input = aoc::parse_input_file_lines_from_args("\n").unwrap();
    run(&input);
}
