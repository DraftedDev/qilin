use crate::game::context::GameContext;

use crate::render::canvas::Canvas;

pub trait Scene {
    fn new() -> Self where Self: Sized;
    fn enter(&mut self);
    fn update(&mut self, canvas: &mut Canvas, ctx: &mut GameContext);
    fn exit(&mut self);
}