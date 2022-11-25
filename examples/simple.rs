use rhachis::*;
use verender::{RenderPassBuilder, VeRenderer, VeRendererBuilder};

#[rhachis::run]
struct Simple {
    renderer: VeRenderer,
    models: Vec<usize>,
}

impl Game for Simple {
    fn init(data: &GameData) -> Self {
        let mut renderer =
            VeRendererBuilder::render_pass_builder(RenderPassBuilder::label("Test")).build(data);
        let models = renderer.load_gltf("examples/simple.gltf", 0).unwrap();

        Self {
            renderer,
            models,
        }
    }

    fn get_renderer(&mut self) -> &mut dyn rhachis::graphics::Renderer {
        &mut self.renderer
    }
}
