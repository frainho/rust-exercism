#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction: match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction: match self.direction {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
        }
    }

    pub fn advance(self) -> Self {
        Self {
            x: match self.direction {
                Direction::West => self.x - 1,
                Direction::East => self.x + 1,
                _ => self.x,
            },
            y: match self.direction {
                Direction::North => self.y + 1,
                Direction::South => self.y - 1,
                _ => self.y,
            },
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |current_robot, c| match c {
            'A' => current_robot.advance(),
            'R' => current_robot.turn_right(),
            'L' => current_robot.turn_left(),
            _ => unreachable!(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
