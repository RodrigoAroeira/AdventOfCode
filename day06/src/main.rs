// https://adventofcode.com/2024/day/6

/*
* I think I definetely over engineered this problem
*/

use std::fmt::Display;

fn main() -> anyhow::Result<()> {
    let filename = std::env::args().nth(1).unwrap_or("input.in".to_string());

    let txt = utility::get_raw_text(&filename)?;
    let lines = utility::get_vec_from_txt(&txt);

    let mut map = Map::try_from(lines)
        .unwrap_or_else(|e| panic!("Unable to convert {filename} content into a map: {e}"));

    while map.guard.is_some() {
        map.transition_state();
        // println!("{map}");
        // println!("{}", "-".repeat(map.map.len() + 9));
    }

    let x_count = map.count_visited();
    println!("The number of X's is: {x_count}");

    Ok(())
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn movement_vector(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }

    fn next_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let s = match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            invalid => return Err(format!("character '{invalid}' isn't a valid character")),
        };

        Ok(s)
    }
}

#[derive(Clone)]
struct Guard {
    direction: Direction,
    pos: (i32, i32),
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = self.direction.next_clockwise();
    }
}

struct Map {
    map: Vec<Vec<char>>,
    guard: Option<Guard>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map {
    fn transition_state(&mut self) {
        let Some(guard) = self.guard.as_mut() else {
            return;
        };

        {
            let (dx, dy) = guard.direction.movement_vector();
            let (old_x, old_y) = guard.pos;
            let (new_x, new_y) = (old_x + dx, old_y + dy);

            self.map[old_x as usize][old_y as usize] = 'X';

            if new_x < 0
                || new_y < 0
                || new_x >= self.map.len() as i32
                || new_y >= self.map[0].len() as i32
            {
                self.guard = None;
                return;
            }

            if self.map[new_x as usize][new_y as usize] == '#' {
                guard.turn_right();
                return;
            }
            guard.pos = (new_x, new_y);

            // todo!("Implement the transition state logic");
        }
    }

    fn count_visited(&self) -> usize {
        self.map.iter().flatten().filter(|&&c| c == 'X').count()
    }
}

impl TryFrom<Vec<String>> for Map {
    type Error = String;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let map = value
            .into_iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut guard = None;
        for (i, row) in map.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if let Ok(direction) = Direction::try_from(c) {
                    let g = Guard {
                        direction,
                        pos: (i as i32, j as i32),
                    };

                    guard = Some(g);
                }
            }
        }

        if guard.is_none() {
            return Err("No guards found".to_string());
        }

        Ok(Self { map, guard })
    }
}
