use crate::game::context::GameContext;
use crate::plugin::QPlugin;
use crate::render::canvas::Canvas;
use crate::scene::Scene;
use crate::types::GameConfig;
use minifb::Window;
use std::time::{Duration, Instant};

/// Main Game initiator to run window and enter scenes.
pub struct Game {
    config: GameConfig,
    scene: Box<dyn Scene>,
    plugins: Vec<Box<dyn QPlugin>>,
}

impl Game {
    /// Create a new game with given entry scene.
    #[inline]
    pub fn new<S: Scene + 'static>() -> Self
    where
        Self: Sized,
    {
        let mut scene = S::new();
        scene.enter();

        Self {
            config: GameConfig::default(),
            scene: Box::new(scene),
            plugins: Vec::new(),
        }
    }

    /// Register a new plugin.
    #[inline]
    pub fn with_plugin<P: QPlugin + 'static>(mut self, plugin: P) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Use given [GameConfig] as config.
    #[inline]
    pub fn with_config(mut self, config: GameConfig) -> Self {
        self.config = config;
        self
    }

    /// Enter new [Scene].
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

    /// Run game.
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

        let fixed_time_step = self.config.fixed_time_step.0;

        let mut last_time = Instant::now();
        let mut accumulated_time = Duration::from_secs(0);

        // Enter all plugin states
        self.plugins.iter_mut().for_each(|pl| {
            pl.on_enter(&mut canvas);
        });

        while window.is_open() {
            let ctx = &mut GameContext::new(&mut window);
            let current_time = Instant::now();
            let delta_time = current_time - last_time;
            last_time = current_time;
            accumulated_time += delta_time;

            canvas.cleanse();

            // call plugins before update
            self.plugins.iter_mut().for_each(|pl| {
                pl.pre_update(&mut canvas, ctx);
            });

            self.scene.update(&mut canvas, ctx);

            // call plugins after update
            self.plugins.iter_mut().for_each(|pl| {
                pl.post_update(&mut canvas, ctx);
            });

            // Call fixed_update multiple times if necessary.
            while accumulated_time >= fixed_time_step {
                self.scene.fixed_update(&mut canvas, ctx);
                accumulated_time -= fixed_time_step;
            }

            window.update_with_buffer(&canvas.clone().buffer(), *width, *height)?;
        }

        // make sure the last scene also calls exit()'s
        self.scene.exit();

        // Exit all plugin states
        self.plugins.iter_mut().for_each(|pl| {
            pl.on_exit(&mut canvas);
        });

        return Ok(());
    }
}
