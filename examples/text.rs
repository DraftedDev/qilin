//! Demonstrates drawing text on the canvas

use qilin::game::context::GameContext;
use qilin::game::game::Game;
use qilin::render::canvas::Canvas;
use qilin::render::color::Color;

use qilin::scene::Scene;
use qilin::simplified::vec2;
use qilin::types::{GameConfig, FPS30};
use qilin::ScaleMode;
use qilin::text::TextSketch;
use qilin::WindowOptions;

struct TextScene;

impl Scene for TextScene {
    // create new empty scene
    fn new() -> Self
        where
            Self: Sized,
    {
        Self
    }

    // gets called when game enters current scene
    fn enter(&mut self) { println!("What do you call a fake noodle?") }

    // gets called when window requests draw updates
    fn update(&mut self, canvas: &mut Canvas, _ctx: &mut GameContext) {
        canvas.drawable(
            &TextSketch::new(vec2(10, 10), include_bytes!("assets/Roboto-Medium.ttf"))
                .with_color(Color::RED)
                .with_text("Hello World!", 30.0)
        );

        canvas.drawable(
            &TextSketch::new(vec2(10, 100), include_bytes!("assets/Roboto-Medium.ttf"))
                .with_color(Color::YELLOW)
                .with_text("Implementing text-rendering was real pain.", 40.0)
        );
    }

    fn fixed_update(&mut self, _canvas: &mut Canvas, _ctx: &mut GameContext) {
        // Will be called X times per second.
        // This ensures, physics are applied independent of frame-rate.
        // See https://docs.unity3d.com/ScriptReference/MonoBehaviour.FixedUpdate.html for FixedUpdate() in Unity.
    }

    // gets called when game exits current scene
    fn exit(&mut self) { println!("An impasta!") }
}

fn main() {
    Game::new::<TextScene>() // create game object with ShapeScene as entry scene
        .with_config(GameConfig {
            title: "My Texts".to_string(), // set window title
            update_rate_limit: FPS30,       // limit update rate to 30 fps, default is 60 fps
            width: 800,                     // set initial width
            height: 600,                    // set initial height
            fixed_time_step: Default::default(), // for better docs, see GameConfig or examples/move.
            window: WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch, // scale pixels to fit in aspect ratio
                resize: true,                              // make window resizeable
                ..Default::default()
            },
        })
        .play()
        .expect("Failed to play game");
}
