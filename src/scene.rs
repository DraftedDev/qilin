use crate::game::context::GameContext;

use crate::render::canvas::Canvas;

/// Trait to represent a scene in the `Game`.
pub trait Scene {
    fn new() -> Self
    where
        Self: Sized;
    fn enter(&mut self);
    fn update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext);
    fn fixed_update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext);
    fn exit(&mut self);
}
