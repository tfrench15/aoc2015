use std::collections::HashMap;

fn main() {
    let directions = load_input();
    let houses = santa_visits(directions);

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

fn santa_visits(directions: &str) -> usize {
    let mut santa = Location::new(0, 0);
    let mut robo_santa = Location::new(0, 0);
    let mut visited: HashMap<Location, bool> = HashMap::new();

    for dir in directions.chars().step_by(2) {
        match dir {
            '^' => {
                santa.north();
                visited.insert(Location::from(&santa), true);
            },
            'v' => {
                santa.south();
                visited.insert(Location::from(&santa), true);
            },
            '>' => {
                santa.east();
                visited.insert(Location::from(&santa), true);
            }
            '<' => {
                santa.west();
                visited.insert(Location::from(&santa), true);
            },
            _ => continue,
        }
    }

    for dir in directions.chars().skip(1).step_by(2) {
        match dir {
            '^' => {
                robo_santa.north();
                visited.insert(Location::from(&robo_santa), true);
            },
            'v' => {
                robo_santa.south();
                visited.insert(Location::from(&robo_santa), true);
            },
            '>' => {
                robo_santa.east();
                visited.insert(Location::from(&robo_santa), true);
            }
            '<' => {
                robo_santa.west();
                visited.insert(Location::from(&robo_santa), true);
            },
            _ => continue,
        }
    }

    visited.len()
}
