//! Demonstrates common input handling

use minifb::{MouseButton};
use qilin::game::context::GameContext;
use qilin::game::game::Game;
use qilin::render::canvas::Canvas;
use qilin::render::color::Color;
use qilin::render::sketch::Sketch;
use qilin::scene::Scene;
use qilin::simplified::vec2;
use qilin::types::{Button, GameConfig, FPS30};
use qilin::ScaleMode;
use qilin::WindowOptions;

struct InputScene {
    color: Color,
}

impl Scene for InputScene {
    // create new empty scene
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            color: Color::WHITE,
        }
    }

    // gets called when game enters current scene
    fn enter(&mut self) { println!("Click and hover on shapes to change color!") }

    // gets called when window requests draw updates
    fn update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext) {
        let mut sketch = Sketch::new();
        sketch
            .circle(vec2(100, 100), 50, self.color)
            .rect(vec2(200, 200), 200, 10, self.color);

        if ctx.clicked_on_sketch(&sketch, Button::Mouse(MouseButton::Left)) {
            self.color = Color::BLUE;
        } else if ctx.hovering_on_sketch(&sketch) {
            self.color = Color::RED;
        } else {
            self.color = Color::WHITE;
        }

        canvas.draw(&sketch);
    }

    fn fixed_update(&mut self, _canvas: &mut Canvas, _ctx: &mut GameContext) {
        // Will be called X times per second.
        // This ensures, physics are applied independent of frame-rate.
        // See https://docs.unity3d.com/ScriptReference/MonoBehaviour.FixedUpdate.html for FixedUpdate() in Unity.
    }

    // gets called when game exits current scene
    fn exit(&mut self) {}
}

fn main() {
    Game::new::<InputScene>() // create game object with ShapeScene as entry scene
        .with_config(GameConfig {
            title: "My Input".to_string(), // set window title
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
