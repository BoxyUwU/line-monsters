use crate::{texture::Texture, Vertex};
use std::sync::Arc;
use ultraviolet::{Rotor3, Vec3};
use wgpu::Device;

pub struct IndexBuffer(pub wgpu::Buffer, pub u32);
pub struct VertexBuffer(pub wgpu::Buffer);

pub struct Spritebatch {
    device: Arc<Device>,

    pub buffers: Vec<(Arc<Texture>, VertexBuffer, IndexBuffer)>,

    current_texture: Option<Arc<Texture>>,
    vertices: Vec<crate::Vertex>,
    indices: Vec<u16>,
}

impl Spritebatch {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,

            buffers: Vec::new(),

            current_texture: None,

            vertices: Vec::with_capacity(128),
            indices: Vec::with_capacity(128),
        }
    }

    pub fn draw(&mut self, position: Vec3, plane: Vec3, texture: Arc<Texture>) {
        assert!(self.vertices.len() + 4 <= u16::MAX as usize);
        assert!(self.indices.len() + 6 <= u32::MAX as usize);

        if let Some(cur_texture) = &self.current_texture {
            if cur_texture.id != texture.id {
                self.flush_to_buffer(Some(Arc::clone(&texture)));
            }
        } else {
            self.current_texture = Some(Arc::clone(&texture));
        }

        let rotor = Rotor3::from_rotation_between(Vec3::unit_y(), plane);

        let top_right = rotor * Vec3::unit_x();
        let top_right = top_right * (texture.size.width as f32 / 256.);

        let bottom_left = rotor * Vec3::unit_z();
        let bottom_left = bottom_left * (texture.size.height as f32 / 256.);

        let bottom_right = top_right + bottom_left;

        let vertices: [Vertex; 4] = [
            Vertex {
                position: position.into(),
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: (position + bottom_left).into(),
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: (position + bottom_right).into(),
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: (position + top_right).into(),
                tex_coords: [1.0, 0.0],
            },
        ];

        let cur_idx = self.vertices.len();
        #[rustfmt::skip]
        let mut indices: [u16; 6] = [
            0, 1, 3,
            1, 2, 3,
        ];
        for index in &mut indices {
            *index += cur_idx as u16;
        }

        self.indices.extend_from_slice(&indices);
        self.vertices.extend_from_slice(&vertices);
    }

    /// Flushes the existing vertices and indices into CommandBuffer
    pub fn flush_to_buffer(&mut self, new_texture: Option<Arc<Texture>>) {
        if self.current_texture.is_none() {
            return;
        }

        use wgpu::util::DeviceExt;
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let vertex_buffer = VertexBuffer(vertex_buffer);

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsage::INDEX,
            });
        let index_buffer = IndexBuffer(index_buffer, self.indices.len() as u32);

        self.buffers.push((
            self.current_texture.take().unwrap(),
            vertex_buffer,
            index_buffer,
        ));

        self.current_texture = new_texture;
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn get_buffer(&mut self) -> Vec<(Arc<Texture>, VertexBuffer, IndexBuffer)> {
        self.flush_to_buffer(None);
        let buffer = self.buffers.drain(..).collect();
        buffer
    }
}
