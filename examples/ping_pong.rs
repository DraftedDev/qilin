//! Simple Ping-Pong Game
//! NOTE: REQUIRES `text` FEATURE!

use qilin::game::context::GameContext;
use qilin::game::game::Game;
use qilin::render::canvas::Canvas;
use std::time::Duration;
use minifb::{Key, WindowOptions};
use mint::Vector2;

use qilin::scene::Scene;
use qilin::types::{GameConfig, FPS60};
use qilin::render::color::Color;
use qilin::render::sketch::Sketch;
use qilin::ScaleMode;
use qilin::simplified::vec2;
use qilin::text::TextSketch;

const SPEED: i32 = 5;
const BALL_SPEED: i32 = 5;
const BALL_RADIUS: i32 = 20;
const PADDLE_HEIGHT: i32 = 100;
const PADDLE_WIDTH: i32 = 10;
const GOAL_HEIGHT: i32 = 100;
const GOAL_WIDTH: i32 = 30;

struct PingPongScene {
    paddle_y: i32,
    ball_pos: Vector2<i32>,
    ball_dir: Vector2<i32>,
    window_width: i32,
    window_height: i32,
    score: i32,
}

impl Scene for PingPongScene {
    fn new() -> Self
        where
            Self: Sized,
    {
        Self {
            paddle_y: 250,
            ball_pos: vec2(400, 300),
            ball_dir: vec2(1, 1),
            window_width: 800,
            window_height: 600,
            score: 0,
        }
    }

    fn enter(&mut self) {
        println!("Ping Pong Game || Made with the Qilin Game Engine");
        println!("-------------------- CONTROL --------------------");
        println!("ESC : Exit Game");
        println!("W : Move Up");
        println!("S : Move Down");
    }

    fn update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext) {
        // score
        canvas.drawable(
            &TextSketch::new(vec2(350, 10), include_bytes!("assets/Roboto-Medium.ttf"))
                .with_text(format!("Score: {}", self.score).as_str(), 20.0)
                .with_color(Color::YELLOW)
        );

        // paddle
        canvas.draw(Sketch::new().rect(
            vec2(50, self.paddle_y as u32),
            PADDLE_WIDTH as u32,
            PADDLE_HEIGHT as u32,
            Color::RED,
        ));

        // ball
        canvas.draw(Sketch::new().circle(
            vec2(self.ball_pos.x as u32, self.ball_pos.y as u32),
            BALL_RADIUS as u32,
            Color::GREEN,
        ));

        // goal
        canvas.draw(Sketch::new().rect(
            vec2((self.window_width - GOAL_WIDTH) as u32, ((self.window_height - GOAL_HEIGHT) / 2) as u32),
            GOAL_WIDTH as u32,
            GOAL_HEIGHT as u32,
            Color::AQUA,
        ));

        // this is not related to physics stuff, so it can be in update()
        if ctx.is_key_down(Key::Escape) {
            ctx.exit();
        }
    }

    fn fixed_update(&mut self, _canvas: &mut Canvas, ctx: &mut GameContext) {
        // Update paddle position
        if ctx.is_key_down(Key::S) && self.paddle_y < self.window_height - PADDLE_HEIGHT - SPEED {
            self.paddle_y += SPEED;
        } else if ctx.is_key_down(Key::W) && self.paddle_y > SPEED {
            self.paddle_y -= SPEED;
        }

        // Move ball
        self.ball_pos.x += self.ball_dir.x * BALL_SPEED;
        self.ball_pos.y += self.ball_dir.y * BALL_SPEED;

        // Bounce ball on the top and bottom window boundaries
        if self.ball_pos.y < BALL_RADIUS || self.ball_pos.y > self.window_height - BALL_RADIUS {
            self.ball_dir.y *= -1;
        }

        // Bounce ball on the right wall
        if self.ball_pos.x > self.window_width - BALL_RADIUS {
            self.ball_dir.x *= -1;
            // Adjust the ball's position so it doesn't go out of the window
            self.ball_pos.x = self.ball_pos.x.clamp(BALL_RADIUS, self.window_width - BALL_RADIUS);
        }

        // Bounce ball on the left wall
        if self.ball_pos.x < BALL_RADIUS {
            self.ball_dir.x *= -1;
            // Adjust the ball's position so it doesn't go out of the window
            self.ball_pos.x = self.ball_pos.x.clamp(BALL_RADIUS, self.window_width - BALL_RADIUS);
            // decrease score
            self.score -= 1;
        }

        // Check collision with paddle
        if self.ball_pos.x <= 50 + PADDLE_WIDTH
            && self.ball_pos.y >= self.paddle_y
            && self.ball_pos.y <= self.paddle_y + PADDLE_HEIGHT
        {
            self.ball_dir.x *= -1;
            // Adjust the ball's position so it doesn't go into the paddle
            self.ball_pos.x = (50 + PADDLE_WIDTH + BALL_RADIUS).max(self.ball_pos.x);
        }

        // Check goal
        if self.ball_pos.x >= self.window_width - GOAL_WIDTH - BALL_RADIUS
            && self.ball_pos.y >= (self.window_height - GOAL_HEIGHT) / 2
            && self.ball_pos.y <= (self.window_height + GOAL_HEIGHT) / 2
        {
            self.ball_pos = vec2(self.window_width / 2, self.window_height / 2);
            self.ball_dir.x *= -1;

            // Increase the score
            self.score += 1;
        }
    }

    fn exit(&mut self) {
        println!("Exiting!");
    }
}

fn main() {
    Game::new::<PingPongScene>()
        .with_config(GameConfig {
            title: "Ping Pong".to_string(),
            width: 800,
            height: 600,
            window: WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch,
                resize: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .play()
        .expect("Failed to play game");
}
