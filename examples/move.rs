use qilin::game::context::GameContext;
use qilin::game::game::Game;
use qilin::render::canvas::Canvas;
use qilin::render::color::Color;
use qilin::render::sketch::Sketch;
use qilin::scene::Scene;
use qilin::simplified::vec2;
use qilin::types::{GameConfig, FPS60};
use qilin::Key;
use qilin::ScaleMode;
use qilin::Vector2;
use qilin::WindowOptions;
use std::time::Duration;

const MAX_Y: u32 = 560;
const MAX_X: u32 = 760;
const MIN_Y: u32 = 40;
const MIN_X: u32 = 40;
const SPEED: u32 = 2;

struct BounceScene {
    pos: Vector2<u32>,
}

impl Scene for BounceScene {
    // create new empty scene
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            pos: vec2(100, 100),
        }
    }

    // gets called when game enters current scene
    fn enter(&mut self) { println!("What do you call a fake noodle?") }

    // gets called when window requests draw updates
    fn update(&mut self, canvas: &mut Canvas, _ctx: &mut GameContext) {
        // draw walls in red
        canvas.draw(
            Sketch::new()
                .line(vec2(10, 10), vec2(790, 10), Color::RED)
                .line(vec2(790, 10), vec2(790, 590), Color::RED)
                .line(vec2(790, 590), vec2(10, 590), Color::RED)
                .line(vec2(10, 590), vec2(10, 10), Color::RED),
        );

        // draw circle/player at position
        canvas.draw(Sketch::new().circle(self.pos, 30, Color::AQUA));
    }

    fn fixed_update(&mut self, _canvas: &mut Canvas, ctx: &mut GameContext) {
        // move player if key pressed and not hitting wall
        {
            if ctx.is_key_down(Key::W) && self.pos.y > MIN_Y {
                self.pos.y -= SPEED;
            }

            if ctx.is_key_down(Key::S) && self.pos.y < MAX_Y {
                self.pos.y += SPEED;
            }

            if ctx.is_key_down(Key::D) && self.pos.x < MAX_X {
                self.pos.x += SPEED;
            }

            if ctx.is_key_down(Key::A) && self.pos.x > MIN_X {
                self.pos.x -= SPEED;
            }
        }
    }

    // gets called when game exits current scene
    fn exit(&mut self) { println!("An impasta!") }
}

fn main() {
    Game::new::<BounceScene>() // create game object with ShapeScene as entry scene
        .with_config(GameConfig {
            title: "Bouncy".to_string(), // set window title
            update_rate_limit: FPS60,    // limit update rate to 30 fps, default is 60 fps
            width: 800,                  // set initial width
            height: 600,                 // set initial height
            fixed_time_step: Duration::from_secs_f32(1.0 / 120.0), // how many times fixed_update will be called, (1.0 / 120.0) means 300 times per second.
            window: WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch, // scale pixels to fit in aspect ratio
                resize: true,                              // make window resizeable
                ..Default::default()
            },
        })
        .play()
        .expect("Failed to play game");
}
