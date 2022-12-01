use super::Test;
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh},
    Context, GameResult,
};

pub struct Meshes;

impl Test for Meshes {
    fn name(&self) -> &'static str {
        "Meshes"
    }

    fn run(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        canvas.draw(
            &Mesh::new_circle(ctx, DrawMode::fill(), [50., 50.], 50., 0.1, Color::RED)?,
            DrawParam::new(),
        );

        Ok(())
    }
}
