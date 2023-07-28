//! Demonstrates how to use the [qilin::audio] module to play awesome sound files.\
//! Requires `audio` feature.

//! Demonstrates how you can implement game logic, like movement.

use qilin::audio::{AudioManager, Panning};
use qilin::game::context::GameContext;
use qilin::game::game::Game;
use qilin::render::canvas::Canvas;
use qilin::scene::Scene;
use qilin::types::{GameConfig, TimeStamp, FPS60};
use qilin::Key;
use qilin::ScaleMode;
use qilin::WindowOptions;
use std::ops::Not;
use std::time::Duration;

struct AudioScene {
    manager: AudioManager,
}

impl Scene for AudioScene {
    // create new empty scene
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            manager: AudioManager::new().unwrap(),
        }
    }

    // gets called when game enters current scene
    fn enter(&mut self) {
        self.manager
            .load("examples/assets/glados.wav", 1.0, Panning::Normal, false)
            .unwrap();

        println!("Press 'p' to play sound.");
        println!("Press 'i' to increase volume.");
        println!("Press 'd' to decrease volume.");
        println!("Press 'r' to pan sound to the right.");
        println!("Press 'l' to pan sound to the left.");
        println!("Press 'e' to toggle reverse.");
    }

    // gets called when window requests draw updates
    fn update(&mut self, _canvas: &mut Canvas, ctx: &mut GameContext) {
        if ctx.is_key_released(Key::P) {
            self.manager.play(0).unwrap();
        }

        // increase volume
        if ctx.is_key_released(Key::I) {
            self.manager
                .set_volume(0, self.manager.get_volume(0).unwrap() + 1.0);
        }

        // decrease volume
        if ctx.is_key_released(Key::D) {
            self.manager
                .set_volume(0, self.manager.get_volume(0).unwrap() - 1.0);
        }

        // pan hard right
        if ctx.is_key_released(Key::R) {
            self.manager.set_panning(0, Panning::HardRight).unwrap();
        }

        // pan hard left
        if ctx.is_key_released(Key::L) {
            self.manager.set_panning(0, Panning::HardLeft).unwrap();
        }

        // toggle reverse
        if ctx.is_key_released(Key::E) {
            self.manager
                .set_reverse(0, self.manager.get_reverse(0).unwrap().not());
        }
    }

    fn fixed_update(&mut self, _canvas: &mut Canvas, _ctx: &mut GameContext) {}

    // gets called when game exits current scene
    fn exit(&mut self) { println!("Exiting!") }
}

fn main() {
    Game::new::<AudioScene>() // create game object with ShapeScene as entry scene
        .with_config(GameConfig {
            title: "Audio".to_string(), // set window title
            update_rate_limit: FPS60,    // limit update rate to 30 fps, default is 60 fps
            width: 800,                  // set initial width
            height: 600,                 // set initial height
            fixed_time_step: TimeStamp(Duration::from_secs_f32(1.0 / 120.0)), // how many times fixed_update will be called, (1.0 / 120.0) means 120 times per second.
            window: WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch, // scale pixels to fit in aspect ratio
                resize: true,                              // make window resizeable
                ..Default::default()
            },
        })
        .play()
        .expect("Failed to play game");
}
