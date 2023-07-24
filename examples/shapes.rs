//! Demonstrates common shape drawing.

use qilin::game::context::GameContext;
use qilin::game::game::Game;
use qilin::render::canvas::Canvas;
use qilin::render::color::Color;
use qilin::render::sketch::Sketch;
use qilin::scene::Scene;
use qilin::simplified::vec2;
use qilin::types::{GameConfig, FPS30};
use qilin::ScaleMode;
use qilin::WindowOptions;


struct ShapeScene;

impl Scene for ShapeScene {
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
        // draw a sketch containing a single shape/line
        canvas.draw(
            Sketch::new()
                // draw line from (0, 0) to (300, 600) with color green
                .line(vec2(0, 0), vec2(300, 600), Color::GREEN),
        );

        // draw a sketch containing multiple shapes
        canvas.draw(
            Sketch::new()
                // draw circle at (100, 100) with radius of 30 and color red
                .circle(vec2(100, 100), 30, Color::RED)
                // draw rectangle at (300, 300) with width of 100 and height of 100 and color blue
                .rect(vec2(300, 300), 100, 100, Color::BLUE)
                .oval(vec2(400, 200), 120, 60, Color::CYAN),
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
    Game::new::<ShapeScene>() // create game object with ShapeScene as entry scene
        .with_config(GameConfig {
            title: "My Shapes".to_string(), // set window title
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
