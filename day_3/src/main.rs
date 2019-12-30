use std::collections::HashMap;

fn main() {
    let directions = load_input();
    let houses = visit_houses(directions);

    println!("visited {} houses", houses);
}

fn load_input() -> &'static str {
    include_str!("input.txt")
}

#[derive(Debug, Hash, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Location { x, y }
    }

    fn from(other: &Location) -> Self {
        Location {
            x: other.x,
            y: other.y,
        }
    }

    fn north(&mut self) {
        self.y += 1;
    }

    fn south(&mut self) {
        self.y -= 1;
    }

    fn east(&mut self) {
        self.x += 1;
    }

    fn west(&mut self) {
        self.x -= 1;
    }
}

impl Eq for Location {}

fn visit_houses(directions: &str) -> usize {
    let mut current = Location::new(0, 0);
    let mut visited: HashMap<Location, bool> = HashMap::new();

    for dir in directions.chars() {
        match dir {
            '^' => {
                current.north();
                visited.insert(Location::from(&current), true);
            },
            'v' => {
                current.south();
                visited.insert(Location::from(&current), true);
            },
            '>' => {
                current.east();
                visited.insert(Location::from(&current), true);
            }
            '<' => {
                current.west();
                visited.insert(Location::from(&current), true);
            },
            _ => continue,
        }
    }

    visited.len()
}