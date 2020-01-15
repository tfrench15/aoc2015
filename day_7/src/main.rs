fn main() {
    println!("Hello, world!");
}

enum Operation {
    And,
    Or,
    Not,
    LShift,
    RShift,
}

fn load_input() -> &'static str {
    include_str!("input.txt")
}

fn parse_input(input: &str) -> {
    let gates: Vec<&str> = input
        .split("\n")
        .collect();
}

