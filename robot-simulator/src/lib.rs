// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    drc: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            drc: d,
        }
    }

    pub fn turn_right(self) -> Self {
        match self.drc {
            Direction::North => Robot {
                drc: Direction::East,
                ..self
            },
            Direction::East => Robot {
                drc: Direction::South,
                ..self
            },
            Direction::South => Robot {
                drc: Direction::West,
                ..self
            },
            Direction::West => Robot {
                drc: Direction::North,
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.drc {
            Direction::North => Robot {
                drc: Direction::West,
                ..self
            },
            Direction::East => Robot {
                drc: Direction::North,
                ..self
            },
            Direction::South => Robot {
                drc: Direction::East,
                ..self
            },
            Direction::West => Robot {
                drc: Direction::South,
                ..self
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.drc {
            Direction::North => Robot {
                pos: (self.pos.0, self.pos.1 + 1),
                ..self
            },
            Direction::East => Robot {
                pos: (self.pos.0 + 1, self.pos.1),
                ..self
            },
            Direction::South => Robot {
                pos: (self.pos.0, self.pos.1 - 1),
                ..self
            },
            Direction::West => Robot {
                pos: (self.pos.0 - 1, self.pos.1),
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |res, action| match action {
            'R' => res.turn_right(),
            'L' => res.turn_left(),
            'A' => res.advance(),
            _ => panic!("err input instruction!"),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.drc
    }
}
