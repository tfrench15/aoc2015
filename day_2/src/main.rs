fn main() {
    let gifts = load_input();
    let mut wrapping_paper = 0;
    let mut ribbon = 0;

    for gift in &gifts {
        wrapping_paper += gift.wrapping_paper();
        ribbon += gift.ribbon();
    }

    println!("required wrapping paper is {}", wrapping_paper);
    println!("required ribbon is {}", ribbon);
}

#[derive(Debug)]
struct Gift(u64, u64, u64);

impl Gift {
    fn new(dimensions: &[&str]) -> Self {
        let mut digits: Vec<u64> = dimensions
            .iter()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        
        digits.sort();

        Gift(digits[0], digits[1], digits[2])
    }

    fn surface_area(&self) -> u64 {
        let first = 2 * self.0 * self.1;
        let second = 2 * self.0 * self.2;
        let third = 2 * self.1 * self.2;

        first + second + third
    }

    fn area_of_smallest_side(&self) -> u64 {
        self.0 * self.1
    }

    fn wrapping_paper(&self) -> u64 {
        self.surface_area() + self.area_of_smallest_side()
    }

    fn shortest_perimeter(&self) -> u64 {
        2 * self.0 + 2 * self.1
    }

    fn volume(&self) -> u64 {
        self.0 * self.1 * self.2
    }

    fn ribbon(&self) -> u64 {
        self.shortest_perimeter() + self.volume()
    }
}

fn load_input() -> Vec<Gift> {
    let input = include_str!("input.txt");
    let mut gifts = Vec::new();

    let dimensions: Vec<&str> = input
        .split('\n')
        .filter(|dim| !dim.is_empty())
        .collect();
    
    for dim in dimensions {
        let lwh: Vec<&str> = dim
            .split('x')
            .filter(|b| !b.is_empty())
            .collect();
        
        let gift = Gift::new(&lwh);
        gifts.push(gift);
    }

    gifts
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_first_example() {
        let gift = super::Gift(2, 3, 4);

        assert_eq!(gift.surface_area() + gift.area_of_smallest_side(), 58);
    }

    #[test]
    fn test_second_example() {
        let gift = super::Gift(1, 1, 10);

        assert_eq!(gift.surface_area() + gift.area_of_smallest_side(), 43);
    }

    #[test]
    fn test_smallest_area_one() {
        let gift = super::Gift(1, 1, 10);

        assert_eq!(gift.area_of_smallest_side(), 1);
    }

    #[test]
    fn test_smallest_area_two() {
        let gift = super::Gift(2, 3, 4);

        assert_eq!(gift.area_of_smallest_side(), 6);
    }
}