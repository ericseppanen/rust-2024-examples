pub enum Piece {
    // TODO
}

pub struct Board {
    squares: Vec<Option<Piece>>,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_moves(_piece: &Option<Piece>) -> Vec<Direction> {
    todo!()
}

pub fn possible_moves(board: &Board) -> impl Iterator<Item = Direction> {
    let moves = board
        .squares
        .iter()
        .flat_map(find_moves)
        .collect::<Vec<_>>();
    moves.into_iter()
}
