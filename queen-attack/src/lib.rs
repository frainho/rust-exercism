#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let queen_position = &self.0;
        let other_queen_position = &other.0;
        let same_file = queen_position.file == other_queen_position.file;
        let same_rank = queen_position.rank == other_queen_position.rank;
        if same_file || same_rank || self.in_diagonal(other_queen_position) {
            true
        } else {
            false
        }
    }

    fn in_diagonal(&self, other_queen_position: &ChessPosition) -> bool {
        let queen_position = &self.0;
        let rank_diference = (queen_position.rank - other_queen_position.rank).abs();
        let file_diference = (queen_position.file - other_queen_position.file).abs();
        rank_diference == file_diference
    }
}
