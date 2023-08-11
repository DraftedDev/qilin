use crate::game::context::GameContext;

use crate::render::canvas::Canvas;


pub trait QPlugin {
    /// Gets called before the game enters the main loop.
    fn on_enter(&mut self, canvas: &mut Canvas);

    /// Gets called before the game calls the [Scene::update] method.
    fn pre_update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext);

    /// Gets called after the game calls the [Scene::update] method.
    fn post_update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext);

    /// Gets called when the game is exiting.
    fn on_exit(&mut self, canvas: &mut Canvas);
}
