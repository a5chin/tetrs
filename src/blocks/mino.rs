use std::vec;

#[derive(Debug)]
pub enum Kind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
pub struct Controller {
    pos: (isize, isize),
    mino: Mino,
    direction: Direction,
}

#[derive(Debug)]
pub struct Mino {
    pub kind: Kind,
    pub shape: Vec<Vec<u8>>,
}

pub fn new(kind: Kind) -> Controller {
    let shape = match kind {
        Kind::I => vec![vec![0; 4], vec![1; 4], vec![0; 4], vec![0; 4]],
        Kind::J => vec![vec![1, 0, 0], vec![1; 3], vec![0; 3]],
        Kind::L => vec![vec![0, 0, 1], vec![1; 3], vec![0; 3]],
        Kind::O => vec![vec![1; 2]; 2],
        Kind::S => vec![vec![0, 1, 1], vec![1; 3], vec![0, 3]],
        Kind::T => vec![vec![0, 1, 0], vec![1; 3], vec![0; 3]],
        Kind::Z => vec![vec![1, 1, 0], vec![0, 1, 1], vec![0; 3]],
    };
    let mino: Mino = Mino {
        kind: kind,
        shape: shape,
    };

    Controller {
        mino: mino,
        pos: (0, 0),
        direction: Direction::North,
    }
}
