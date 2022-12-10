use std::collections::HashSet;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub visited: HashSet<Location>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Node {
    pub fn new(x: i32, y: i32) -> Node {
        Node {
            x,
            y,
            visited: HashSet::from([Location { x, y }]),
        }
    }

    pub fn move_relative(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    // These _loc functions exist because the borrow checker is a b****
    #[allow(dead_code)]
    fn is_close(&self, other: &Node) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff <= 1 && y_diff <= 1
    }

    fn is_close_loc(&self, other: &Location) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff <= 1 && y_diff <= 1
    }

    pub fn follow(&mut self, other: &Node) {
        self.follow_loc(&Location {
            x: other.x,
            y: other.y,
        });
    }

    pub fn follow_loc(&mut self, other: &Location) {
        if self.is_close_loc(other) {
            return;
        }
        if self.x == other.x {
            if self.y < other.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.y == other.y {
            if self.x < other.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else {
            if self.x < other.x {
                if self.y < other.y {
                    self.x += 1;
                    self.y += 1;
                } else {
                    self.x += 1;
                    self.y -= 1;
                }
            } else {
                if self.y < other.y {
                    self.x -= 1;
                    self.y += 1;
                } else {
                    self.x -= 1;
                    self.y -= 1;
                }
            }
        }
        self.visited.insert(Location {
            x: self.x,
            y: self.y,
        });
    }
}
