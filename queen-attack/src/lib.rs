#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

const CHESS_SIZE: i32 = 8;
impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && file >= 0 && rank < CHESS_SIZE && file < CHESS_SIZE {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        //皇后A(x,y), B(x', y')，那么 x == x' y==y'或者 ((x-x')/(y-y')).abs() == 1
        self.pos.rank == other.pos.rank
            || self.pos.file == other.pos.file
            || (self.pos.rank - other.pos.rank).abs() == (self.pos.file - other.pos.file).abs()
    }
}
