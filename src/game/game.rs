use minifb::{Window};
use crate::game::context::GameContext;
use crate::render::canvas::Canvas;
use crate::scene::Scene;
use crate::types::GameConfig;

pub struct Game {
    config: GameConfig,
    scene: Box<dyn Scene>,
}

impl Game {

    #[inline]
    pub fn new<S: Scene + 'static>() -> Self where Self: Sized {
        let mut scene = S::new();
        scene.enter();

        Self {
            config: GameConfig::default(),
            scene: Box::new(scene),
        }
    }

    #[inline]
    pub fn with_config(mut self, config: GameConfig) -> Self {
        self.config = config;
        self
    }

    #[inline(never)]
    pub fn enter_scene<S: Scene + 'static>(mut self) -> Self {
        // finish old scene
        self.scene.exit();

        // enter new scene
        let mut scene = S::new();
        scene.enter();
        self.scene = Box::new(scene);

        self
    }

    #[inline]
    pub fn play(mut self) -> minifb::Result<()> {
        let width = &self.config.width;
        let height = &self.config.height;

        let mut window = Window::new(
            self.config.title.as_str(),
            self.config.width,
            self.config.height,
            self.config.window,
        )?;

        window.limit_update_rate(Some(self.config.update_rate_limit));

        let mut canvas = Canvas::new(*width, *height);

        while window.is_open() {
            self.scene.update(&mut canvas, &mut GameContext::new(&mut window));
            window.update_with_buffer(&canvas.clone().buffer(), *width, *height)?;
        }

        return Ok(());
    }

}