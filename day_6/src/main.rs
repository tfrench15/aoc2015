use std::collections::HashMap;

fn main() {
    println!("hello world!");
}

#[derive(Debug)]
struct LightGrid {
    lights: HashMap<Light, LightStatus>,
}

impl LightGrid {
    fn new() -> Self {
        LightGrid {
            lights: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Light {
    x: u32,
    y: u32,
}

impl Light {
    fn new(x: u32, y: u32) -> Self {
        Light {
            status: LightStatus::Off,
            x, 
            y,
        }
    }

    fn toggle(&mut self) {
        match self.status {
            LightStatus::On => {
                self.status = LightStatus::Off;
            },
            LightStatus::Off => {
                self.status = LightStatus::On;
            },
        };
    }

    fn turn_on(&mut self) {
        self.status = LightStatus::On;
    }

    fn turn_off(&mut self) {
        self.status = LightStatus::Off;
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
    include_str!("input.txt");
}

fn parse_input(input: &str) -> Vec<