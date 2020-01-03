use std::collections::HashMap;

fn main() {
    let input = load_input();
    let instructions = parse_input(input);
    let mut grid = LightGrid::new();

    for instruction in instructions {
        grid.decorate(&instruction);
    }

    let mut brightness = 0;
    for val in grid.lights.values() {
        brightness += val.0;
    }

    println!("brightness is: {}", brightness);
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    x_range: (usize, usize),
    y_range: (usize, usize),
}

impl Instruction {
    fn new(action: Action, x_range: (usize, usize), y_range: (usize, usize)) -> Self {
        Instruction {
            action,
            x_range, 
            y_range,
        }
    }
}

#[derive(Debug)]
struct LightGrid {
    lights: HashMap<Light, LightStatus>,
}

impl LightGrid {
    fn new() -> Self {
        let mut grid = LightGrid {
            lights: HashMap::new(),
        };

        for i in 0..1000 {
            for j in 0..1000 {
                let light = Light::new(i, j);
                grid.lights.insert(light, LightStatus(0));
            }
        }

        grid
    }

    fn toggle(&mut self, light: &Light) {
        if let Some(l) = self.lights.get_mut(light) {
            l.0 += 2;
        }
    }

    fn turn_off(&mut self, light: &Light) {
        if let Some(l) = self.lights.get_mut(light) {
            if l.0 > 0 {
                l.0 -= 1;
            }
        }
    }

    fn turn_on(&mut self, light: &Light) {
        if let Some(l) = self.lights.get_mut(light) {
            l.0 += 1;
        }
    }

    fn decorate(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::Toggle => {
                for i in instruction.x_range.0..=instruction.x_range.1 {
                    for j in instruction.y_range.0..=instruction.y_range.1 {
                        let light = Light::new(i, j);
                        self.toggle(&light);
                    }
                }
            },
            Action::TurnOn => {
                for i in instruction.x_range.0..=instruction.x_range.1 {
                    for j in instruction.y_range.0..=instruction.y_range.1 {
                        let light = Light::new(i, j);
                        self.turn_on(&light);
                    }
                }
            },
            Action::TurnOff => {
                for i in instruction.x_range.0..=instruction.x_range.1 {
                    for j in instruction.y_range.0..=instruction.y_range.1 {
                        let light = Light::new(i, j);
                        self.turn_off(&light);
                    }
                }
            },
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Light {
    x: usize,
    y: usize,
}

impl Light {
    fn new(x: usize, y: usize) -> Self {
        Light { x, y }
    }
}

#[derive(Debug)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

#[derive(Debug)]
struct LightStatus(u32);

fn load_input() -> &'static str {
    include_str!("input.txt")
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut parsed_instructions = Vec::new();

    let instructions: Vec<&str> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect();

    for instruction in instructions {
        if instruction.starts_with("turn on") {
            let trimmed = instruction.trim_start_matches("turn on ");
            let split: Vec<&str> = trimmed
                .splitn(2, "through")
                .collect();

            let start: Vec<usize> = split[0]
                .split(",")
                .map(|i| i.trim())
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            
            let end: Vec<usize> = split[1]
                .split(",")
                .map(|i| i.trim())
                .map(|i| i.parse::<usize>().unwrap())
                .collect();

            let x_range;
            let y_range;

            if start[0] < end[0] {
                x_range = (start[0], end[0]);
            } else {
                x_range = (end[0], start[0]);
            }

            if start[1] < end[1] {
                y_range = (start[1], end[1]);
            } else {
                y_range = (end[1], start[1]);
            }

            let parsed_instruction = Instruction::new(Action::TurnOn, x_range, y_range);
            parsed_instructions.push(parsed_instruction);

        } else if instruction.starts_with("turn off") {
            let trimmed = instruction.trim_start_matches("turn off ");
            let split: Vec<&str> = trimmed
                .splitn(2, "through")
                .collect();

            let start: Vec<usize> = split[0]
                .split(",")
                .map(|i| i.trim())
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            
            let end: Vec<usize> = split[1]
                .split(",")
                .map(|i| i.trim())
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            
            let x_range;
            let y_range;

            if start[0] < end[0] {
                x_range = (start[0], end[0]);
            } else {
                x_range = (end[0], start[0]);
            }

            if start[1] < end[1] {
                y_range = (start[1], end[1]);
            } else {
                y_range = (end[1], start[1]);
            }

            let parsed_instruction = Instruction::new(Action::TurnOff, x_range, y_range);
            parsed_instructions.push(parsed_instruction);
        } else {
            let trimmed = instruction.trim_start_matches("toggle ");
            let split: Vec<&str> = trimmed
                .splitn(2, "through")
                .collect();

            let start: Vec<usize> = split[0]
                .split(",")
                .map(|i| i.trim())
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            
            let end: Vec<usize> = split[1]
                .split(",")
                .map(|i| i.trim())
                .map(|i| i.parse::<usize>().unwrap())
                .collect();

            let x_range;
            let y_range;

            if start[0] < end[0] {
                x_range = (start[0], end[0]);
            } else {
                x_range = (end[0], start[0]);
            }

            if start[1] < end[1] {
                y_range = (start[1], end[1]);
            } else {
                y_range = (end[1], start[1]);
            }

            let parsed_instruction = Instruction::new(Action::Toggle, x_range, y_range);
            parsed_instructions.push(parsed_instruction);
        }
    }

    parsed_instructions
}