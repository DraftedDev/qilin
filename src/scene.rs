use crate::game::context::GameContext;

use crate::render::canvas::Canvas;

/// Trait to represent a scene in the `Game`.
pub trait Scene {
    #[inline(never)]
    fn new() -> Self
    where
        Self: Sized;

    #[inline(never)]
    fn enter(&mut self);

    #[inline]
    fn update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext);

    #[inline]
    fn fixed_update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext);

    #[inline(never)]
    fn exit(&mut self);
}
