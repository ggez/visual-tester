use ggez::{graphics::Canvas, Context, GameResult};

pub mod meshes;
pub use meshes::*;

pub mod text;
pub use text::*;

pub mod shader_params;
pub use shader_params::*;

pub fn all_tests() -> [Box<dyn Test>; 3] {
    [Box::new(Meshes), Box::new(Text), Box::new(ShaderParams)]
}

pub trait Test {
    fn name(&self) -> &'static str;
    fn run(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
}
