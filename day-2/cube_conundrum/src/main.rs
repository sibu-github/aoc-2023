use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

impl FromStr for Cubes {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut splitted = s.split(',').map(|s| s.trim());
        while let Some(s) = splitted.next() {
            let (n, color) = s.split_once(' ').unwrap();
            let n = n.parse::<u32>().unwrap();
            match color {
                "red" => red += n,
                "green" => green += n,
                "blue" => blue += n,
                _ => panic!(),
            }
        }
        Ok(Self::new(red, green, blue))
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, sets) = s.split_once(':').unwrap();
        let id = id.strip_prefix("Game").unwrap();
        let id = id.trim().parse::<u32>().unwrap();
        let sets = sets
            .trim()
            .split(';')
            .map(|s| Cubes::from_str(s).unwrap())
            .collect();
        Ok(Self { id, sets })
    }
}

impl Game {
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.sets
            .iter()
            .all(|cubes| cubes.red <= red && cubes.green <= green && cubes.blue <= blue)
    }
    fn min_cubes_power(&self) -> u32 {
        let (red, green, blue) = self.sets.iter().fold((0, 0, 0), |prev, cubes| {
            let r = get_max(prev.0, cubes.red);
            let g = get_max(prev.1, cubes.green);
            let b = get_max(prev.2, cubes.blue);
            (r, g, b)
        });
        red * green * blue
    }
}

fn get_max(v1: u32, v2: u32) -> u32 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let games = content
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .collect::<Vec<_>>();
    let sum = games
        .iter()
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|g| g.id)
        .sum::<u32>();
    println!("{}", sum);
    let sum_powers = games.iter().map(|g| g.min_cubes_power()).sum::<u32>();
    println!("{}", sum_powers);
}
