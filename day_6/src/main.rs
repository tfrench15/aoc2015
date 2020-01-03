use std::collections::HashMap;

fn main() {
    println!("hello world!");
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    start: (usize, usize),
    end: (usize, usize),
}

impl Instruction {
    fn new(action: Action, start: (usize, usize), end: (usize, usize)) -> Self {
        Instruction {
            action,
            start, 
            end,
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
                grid.lights.insert(light, LightStatus::Off);
            }
        }

        grid
    }

    fn toggle(&mut self, light: &Light) {
        if let Some(l) = self.lights.get_mut(light) {
            match *l {
                LightStatus::Off => {
                    *l = LightStatus::On;
                },
                LightStatus::On => {
                    *l = LightStatus::Off;
                },
            }
        }
    }

    fn turn_off(&mut self, light: &Light) {
        if let Some(l) = self.lights.get_mut(light) {
            *l = LightStatus::Off;
        }
    }

    fn turn_on(&mut self, light: &Light) {
        if let Some(l) = self.lights.get_mut(light) {
            *l = LightStatus::On;
        }
    }
}

#[derive(Debug, Hash)]
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
enum LightStatus {
    On,
    Off,
}

fn load_input() -> &'static str {
    include_str!("input.txt")
}

fn parse_input(input: &str) -> Vec<Instruction> {
    unimplemented!("build a vector of Instructions")
}