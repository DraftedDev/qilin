//! Shows how to register and develop Plugins
//! NOTE: REQUIRES `plugin-dev` FEATURE!

use qilin::game::context::GameContext;
use qilin::game::game::Game;

use qilin::render::canvas::Canvas;

use std::time::Duration;

use qilin::scene::Scene;

use qilin::plugin::QPlugin;
use qilin::render::color::Color;
use qilin::render::sketch::Sketch;
use qilin::simplified::vec2;
use qilin::types::{GameConfig, TimeStamp, FPS30};

use qilin::ScaleMode;
use qilin::WindowOptions;

struct PluginScene;

impl Scene for PluginScene {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    // gets called when game enters current scene
    fn enter(&mut self) {}

    // gets called when window requests draw updates
    fn update(&mut self, _canvas: &mut Canvas, _ctx: &mut GameContext) {}

    fn fixed_update(&mut self, _canvas: &mut Canvas, _ctx: &mut GameContext) {}

    // gets called when game exits current scene
    fn exit(&mut self) { println!("Exiting!") }
}

struct MyPlugin;

impl QPlugin for MyPlugin {
    fn on_enter(&mut self, _canvas: &mut Canvas) {
        println!("Hello from MyPlugin!");
    }

    fn pre_update(&mut self, _canvas: &mut Canvas, _ctx: &mut GameContext) {}

    fn post_update(&mut self, canvas: &mut Canvas, _ctx: &mut GameContext) {
        // this cannot be overriden by the game, since it's called after update().
        canvas.draw(Sketch::new().rect(vec2(100, 100), 100, 100, Color::BLUE))
    }

    fn on_exit(&mut self, _canvas: &mut Canvas) {
        println!("Bye from MyPlugin!");
    }
}

fn main() {
    Game::new::<PluginScene>()
        .with_plugin(MyPlugin {})
        .with_config(GameConfig {
            title: "Player Preferences".to_string(), // set window title
            update_rate_limit: FPS30,       // limit update rate to 30 fps, default is 60 fps
            width: 800,                     // set initial width
            height: 600,                    // set initial height
            fixed_time_step: TimeStamp(Duration::from_secs_f32(1.0 / 10.0)), // for better docs, see GameConfig or examples/move.
            window: WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch, // scale pixels to fit in aspect ratio
                resize: true,                              // make window resizeable
                ..Default::default()
            },
        })
        .play()
        .expect("Failed to play game");
}
