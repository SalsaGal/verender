use rhachis::*;
use verender::{RenderPassBuilder, VeRenderer, VeRendererBuilder};

#[rhachis::run]
struct Simple {
    renderer: VeRenderer,
}

impl Game for Simple {
    fn init(data: &GameData) -> Self {
        Self {
            renderer: VeRendererBuilder::render_pass_builder(RenderPassBuilder::label("Test"))
                .build(data),
        }
    }

    fn get_renderer(&mut self) -> &mut dyn rhachis::graphics::Renderer {
        &mut self.renderer
    }
}
