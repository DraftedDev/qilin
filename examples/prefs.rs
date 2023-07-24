//! Shows how to use the `PlayerPrefs` struct.
//! NOTE: REQUIRES `store` FEATURE!

use qilin::game::context::GameContext;
use qilin::game::game::Game;
use qilin::game::store::{PlayerPrefs, Storable};
use qilin::render::canvas::Canvas;
use std::path::PathBuf;
use std::time::Duration;

use qilin::scene::Scene;

use qilin::types::{GameConfig, TimeStamp, FPS30};
use qilin::Key;
use qilin::ScaleMode;
use qilin::WindowOptions;

struct PrefsScene {
    prefs: PlayerPrefs,
}

impl Scene for PrefsScene {
    // create new empty scene
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            // creates player prefs at "player_prefs.json"
            prefs: PlayerPrefs::new(PathBuf::from("player_prefs.json".to_string())),
        }
    }

    // gets called when game enters current scene
    fn enter(&mut self) {
        println!("Press 'q' to increase decrease the count key of player prefs.");
        println!("Press 'e' to increase increase the count key of player prefs.");
        println!("Press 's' to save player prefs.");

        // stores the value "count" in the player prefs
        self.prefs
            .insert("count".to_string(), Storable::Int(0))
            .unwrap();
    }

    // gets called when window requests draw updates
    fn update(&mut self, _canvas: &mut Canvas, _ctx: &mut GameContext) {
        // Will be called every frame.
    }

    fn fixed_update(&mut self, _canvas: &mut Canvas, ctx: &mut GameContext) {
        // Will be called X times per second.
        // This ensures, physics are applied independent of frame-rate.
        // See https://docs.unity3d.com/ScriptReference/MonoBehaviour.FixedUpdate.html for FixedUpdate() in Unity.

        // get the value "count" from the player prefs
        let count = self
            .prefs
            .get("count".to_string())
            .unwrap()
            .as_int()
            .unwrap();

        // mutate the value "count" depending on input, or save the file
        if ctx.is_key_down(Key::Q) {
            self.prefs
                .insert("count".to_string(), Storable::Int(*count - 1))
                .unwrap();
        } else if ctx.is_key_down(Key::E) {
            self.prefs
                .insert("count".to_string(), Storable::Int(*count + 1))
                .unwrap();
        } else if ctx.is_key_down(Key::S) {
            self.prefs.save();
        }
    }

    // gets called when game exits current scene
    fn exit(&mut self) { println!("An impasta!") }
}

fn main() {
    Game::new::<PrefsScene>() // create game object with ShapeScene as entry scene
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
