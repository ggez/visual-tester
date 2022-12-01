use super::Test;
use crevice::std140::AsStd140;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawMode},
    Context, GameResult,
};

#[derive(AsStd140)]
struct Dim {
    rate: f32,
}

pub struct ShaderParams;

impl Test for ShaderParams {
    fn name(&self) -> &'static str {
        "Shader Params"
    }

    fn run(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let dim = Dim { rate: 0.25 };
        let shader = graphics::ShaderBuilder::new_wgsl()
            .fragment_code(include_str!("../../../resources/dimmer.wgsl"))
            .build(&ctx.gfx)?;
        let params = graphics::ShaderParamsBuilder::new(&dim).build(ctx);

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(10.0, 30.0),
            10.0,
            2.0,
            Color::RED,
        )?;
        canvas.draw(&circle, Vec2::new(0.0, 0.0));

        params.set_uniforms(ctx, &dim);
        canvas.set_shader(shader.clone());
        canvas.set_shader_params(params.clone());
        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(40.0, 30.0),
            10.0,
            2.0,
            Color::RED,
        )?;
        canvas.draw(&circle, Vec2::new(0.0, 0.0));

        let uniforms = Dim { rate: 0.75 };
        let params = graphics::ShaderParamsBuilder::new(&uniforms).build(ctx);
        canvas.set_shader_params(params);
        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(70.0, 30.0),
            10.0,
            2.0,
            Color::RED,
        )?;
        canvas.draw(&circle, Vec2::new(0.0, 0.0));

        Ok(())
    }
}
