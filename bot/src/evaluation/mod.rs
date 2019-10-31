use libtetris::{ LockResult, Board, Piece };

mod misalike;
pub use self::misalike::Misalike;
mod standard;
pub use self::standard::{ Standard, has_attack_soon };
pub mod changed;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Evaluation {
    pub accumulated: i32,
    pub transient: i32
}

pub trait Evaluator {
    fn name(&self) -> String;
    fn evaluate(
        &self, lock: &LockResult, board: &Board, move_time: u32, placed: Piece
    ) -> Evaluation;
}
