use board::{GoBoard, Team};

pub type HeuristicFn = fn(board: GoBoard, team: Team) -> i32;

/// Returns a numerical value which approximate how close the board is to
/// victory for the team.
pub fn heuristic(board: GoBoard, team: Team) -> i32 {
    //
    42
}
