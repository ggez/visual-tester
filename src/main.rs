use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::winit::event::VirtualKeyCode;
use ggez::{glam, Context, ContextBuilder, GameResult};
use std::ffi::OsStr;

mod tests;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("ggez_visual_tester", "ggez team")
        .window_setup(WindowSetup {
            title: "(ggez) Visual Tester".into(),
            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let tester = Tester::new(&mut ctx);

    event::run(ctx, event_loop, tester);
}

struct TestResult {
    name: &'static str,
    output: Image,
    expected: Image,
    matches: bool,
}

fn perform_tests(ctx: &mut Context) -> GameResult<Vec<TestResult>> {
    for p in ctx
        .fs
        .read_dir("/difference")?
        .into_iter()
        .filter(|name| name.extension() == Some(OsStr::new("png")))
    {
        ctx.fs.delete(p)?;
    }
    tests::all_tests()
        .into_iter()
        .map(|test| {
            ctx.gfx.begin_frame()?;
            let output = graphics::Image::new_canvas_image(
                ctx,
                graphics::ImageFormat::Rgba8Unorm,
                128,
                128,
                1,
            );
            let mut canvas = graphics::Canvas::from_image(ctx, output.clone(), Color::WHITE);
            test.run(ctx, &mut canvas)?;
            canvas.finish(ctx)?;
            ctx.gfx.end_frame()?;

            let expected_path = format!("/expected/{}.png", test.name());
            let expected = if ctx.fs.exists(&expected_path) {
                Image::from_path(ctx, expected_path)?
            } else {
                Image::from_solid(ctx, 256, Color::from_rgba_u32(0))
            };

            let difference: Vec<u8> = output
                .to_pixels(ctx)?
                .chunks_exact(4)
                .zip(expected.to_pixels(ctx)?.chunks_exact(4))
                .flat_map(|(out, exp)| {
                    if out == exp {
                        [0, 0, 0, 0]
                    } else {
                        [255, 255, 255, 255]
                    }
                })
                .collect();

            let matches = difference.iter().all(|v| *v == 0);

            if !matches {
                let path = format!("/difference/{}-difference.png", test.name());
                Image::from_pixels(
                    ctx,
                    &difference,
                    output.format(),
                    output.width(),
                    output.height(),
                )
                .encode(ctx, graphics::ImageEncodingFormat::Png, path)?;
            }

            Ok(TestResult {
                name: test.name(),
                output,
                expected,
                matches,
            })
        })
        .collect()
}

struct Tester {
    current: usize,
    results: Vec<TestResult>,
}

impl Tester {
    pub fn new(ctx: &mut Context) -> Tester {
        ctx.fs.mount(&std::env::current_dir().unwrap(), false);
        let results = perform_tests(ctx).unwrap();
        Tester {
            results,
            current: 0,
        }
    }
}

impl EventHandler for Tester {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut frame = graphics::Canvas::from_frame(ctx, Color::from_rgb_u32(0xAABBCC));
        frame.set_sampler(graphics::Sampler::nearest_clamp());
        frame.draw(
            &graphics::Text::new("Press O to overwrite ALL the expected outputs"),
            DrawParam::new().color(Color::RED).dest(glam::vec2(8., 8.)),
        );

        let current = &self.results[self.current];
        frame.draw(
            &graphics::Text::new(format!("Currently viewing '{}'", current.name)),
            DrawParam::new()
                .dest(glam::vec2(ctx.gfx.drawable_size().0 / 2., 36.))
                .offset([0.5, 0.5]),
        );

        frame.draw(
            &graphics::Text::new("Output"),
            DrawParam::new()
                .dest(glam::vec2(ctx.gfx.drawable_size().0 / 3., 60.))
                .offset([0.5, 0.5]),
        );
        frame.draw(
            &graphics::Text::new("Expected"),
            DrawParam::new()
                .dest(glam::vec2(ctx.gfx.drawable_size().0 * 2. / 3., 60.))
                .offset([0.5, 0.5]),
        );
        frame.draw(
            &current.output,
            DrawParam::new()
                .dest(glam::vec2(ctx.gfx.drawable_size().0 / 3., 70.))
                .offset([0.5, 0.]),
        );
        frame.draw(
            &current.expected,
            DrawParam::new()
                .dest(glam::vec2(ctx.gfx.drawable_size().0 * 2. / 3., 70.))
                .offset([0.5, 0.]),
        );

        frame.draw(
            &graphics::Text::new(if current.matches {
                "matches!"
            } else {
                "different.."
            }),
            DrawParam::new()
                .color(if current.matches {
                    Color::GREEN
                } else {
                    Color::RED
                })
                .dest(glam::vec2(ctx.gfx.drawable_size().0 / 2., 360.))
                .offset([0.5, 0.5]),
        );

        frame.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> GameResult {
        if repeated {
            return Ok(());
        }

        if input.keycode == Some(VirtualKeyCode::O) {
            print!("Overwriting expected outputs");
            for result in &self.results {
                print!(".");
                let output = format!("/expected/{}.png", result.name);
                result
                    .output
                    .encode(ctx, graphics::ImageEncodingFormat::Png, output)?;
            }
            println!();
            println!("Updating test results..");
            self.results = perform_tests(ctx)?;
            println!("Finished!");
        }
        if input.keycode == Some(VirtualKeyCode::Left) {
            if self.current == 0 {
                self.current = self.results.len() - 1;
            } else {
                self.current -= 1;
            }
        }
        if input.keycode == Some(VirtualKeyCode::Right) {
            if self.current == self.results.len() - 1 {
                self.current = 0;
            } else {
                self.current += 1;
            }
        }
        Ok(())
    }
}
