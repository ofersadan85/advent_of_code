#[derive(Debug)]
pub struct GameResult {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl TryFrom<&str> for GameResult {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for item in s.trim().split(", ") {
            let (count, color) = item.split_once(' ').expect("count and color");
            let count = count.parse::<u32>().expect("count parse");
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => Err("unknown color")?,
            }
        }
        Ok(Self { red, blue, green })
    }
}

impl GameResult {
    const fn is_possible(&self, another: &Self) -> bool {
        self.red <= another.red && self.blue <= another.blue && self.green <= another.green
    }
}

#[derive(Debug)]
pub struct Game {
    pub index: u32,
    pub results: Vec<GameResult>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (game_title, results) = s.trim().split_once(": ").expect("game title");
        let index = game_title
            .split_once(' ')
            .expect("game index")
            .1
            .parse::<u32>()
            .expect("game index parse");
        let results = results
            .split("; ")
            .filter_map(|line| GameResult::try_from(line).ok())
            .collect::<Vec<GameResult>>();
        Self { index, results }
    }
}

impl Game {
    fn is_possible(&self, another: &GameResult) -> bool {
        self.results
            .iter()
            .all(|result| result.is_possible(another))
    }

    fn power(&self) -> u32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for result in &self.results {
            if result.red > max_red {
                max_red = result.red;
            }
            if result.green > max_green {
                max_green = result.green;
            }
            if result.blue > max_blue {
                max_blue = result.blue;
            }
        }
        max_red * max_green * max_blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let s = "Game 1: 7 red, 14 blue; 2 blue, 3 red, 3 green; 4 green, 12 blue, 15 red; 3 green, 12 blue, 3 red; 11 red, 2 green";
        let game = Game::from(s);
        assert_eq!(game.index, 1);
        assert_eq!(game.results.len(), 5);
        assert_eq!(game.results[0].red, 7);
        assert_eq!(game.results[0].blue, 14);
        assert_eq!(game.results[0].green, 0);
        assert_eq!(game.results[1].red, 3);
        assert_eq!(game.results[1].blue, 2);
        assert_eq!(game.results[1].green, 3);
        assert_eq!(game.results[2].red, 15);
        assert_eq!(game.results[2].blue, 12);
        assert_eq!(game.results[2].green, 4);
        assert_eq!(game.results[3].red, 3);
        assert_eq!(game.results[3].blue, 12);
        assert_eq!(game.results[3].green, 3);
        assert_eq!(game.results[4].red, 11);
        assert_eq!(game.results[4].blue, 0);
        assert_eq!(game.results[4].green, 2);
    }

    #[test]
    fn test_game_result_is_possible() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let another = GameResult {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut total = 0;
        for (i, line) in example.lines().enumerate() {
            let game = Game::from(line);
            if [0, 1, 4].contains(&i) {
                total += game.index;
                assert!(game.is_possible(&another), "line: {i}");
            } else {
                assert!(!game.is_possible(&another), "line: {i}");
            }
        }
        assert_eq!(total, 8);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/2023/day02.txt");
        let another = GameResult {
            red: 12,
            green: 13,
            blue: 14,
        };
        let total = input
            .lines()
            .map(Game::from)
            .filter(|game| game.is_possible(&another))
            .map(|game| game.index)
            .sum::<u32>();
        assert_eq!(total, 2204);
    }

    #[test]
    fn test_power() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result: u32 = example.lines().map(|s| Game::from(s).power()).sum();
        assert_eq!(result, 2286);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/2023/day02.txt");
        let result: u32 = input.lines().map(|s| Game::from(s).power()).sum();
        assert_eq!(result, 71036);
    }
}
