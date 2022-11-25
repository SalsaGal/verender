pub mod model;

use model::{Material, Model};
use rhachis::{graphics::Renderer, GameData, IdMap};
use wgpu::TextureView;

macro_rules! builder_option {
    {$($ident: ident: $type: ty), *$(,)?} => {
        $(
            pub fn $ident($ident: $type) -> Self {
                Self {
                    $ident: Some($ident),
                    ..Default::default()
                }
            }
        )*
    };
}

macro_rules! builder {
    {$($ident: ident: $type: ty), *$(,)?} => {
        $(
            pub fn $ident($ident: $type) -> Self {
                Self {
                    $ident,
                    ..Default::default()
                }
            }
        )*
    };
}

pub struct VeRenderer {
    pub render_pass_builder: RenderPassBuilder,
    pub models: IdMap<Model>,
    pub materials: IdMap<Material>,
}

impl Renderer for VeRenderer {
    fn make_render_pass<'a>(
        &'a self,
        view: &'a TextureView,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self.render_pass_builder.label,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: self.render_pass_builder.depth_texture.as_ref().map(|view| {
                wgpu::RenderPassDepthStencilAttachment {
                    view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }
            }),
        })
    }
}

#[derive(Default)]
pub struct VeRendererBuilder {
    pub render_pass_builder: Option<RenderPassBuilder>,
    pub models: IdMap<Model>,
    pub materials: IdMap<Material>,
}

impl VeRendererBuilder {
    pub fn build(self, _data: &GameData) -> VeRenderer {
        VeRenderer {
            render_pass_builder: self.render_pass_builder.unwrap_or_default(),
            materials: self.materials,
            models: self.models,
        }
    }

    builder_option! {
        render_pass_builder: RenderPassBuilder,
    }
    builder! {
        models: IdMap<Model>,
        materials: IdMap<Material>,
    }
}

#[derive(Default)]
pub struct RenderPassBuilder {
    pub label: Option<&'static str>,
    pub depth_texture: Option<TextureView>,
}

impl RenderPassBuilder {
    builder_option! {
        label: &'static str,
        depth_texture: TextureView,
    }
}
