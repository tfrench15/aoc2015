
fn main() {
    let directions = load_input();
    let position = find_floor(directions);

    println!("Santa enters the basement at position: {:?}", position);
}

fn load_input() -> &'static str {
    include_str!("input.txt")
}

fn find_floor(directions: &str) -> Option<usize> {
    let mut floor = 0;

    for (idx, ch) in directions.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("not a valid direction"),
        }

        if floor == -1 {
            return Some(idx + 1)
        }
    }

    None
}
