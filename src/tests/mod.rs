use ggez::{graphics::Canvas, Context, GameResult};

pub mod meshes;
pub use meshes::*;

pub mod text;
pub use text::*;

pub fn all_tests() -> [Box<dyn Test>; 2] {
    [Box::new(Meshes), Box::new(Text)]
}

pub trait Test {
    fn name(&self) -> &'static str;
    fn run(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
}
