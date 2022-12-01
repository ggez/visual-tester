use super::Test;
use ggez::{
    graphics::{self, Canvas, Color, DrawParam, TextFragment},
    Context, GameResult,
};

pub struct Text;

impl Test for Text {
    fn name(&self) -> &'static str {
        "Text"
    }

    fn run(&self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        canvas.draw(
            &graphics::Text::new(TextFragment::new("Hello World!").color(Color::RED)),
            DrawParam::new(),
        );

        Ok(())
    }
}
