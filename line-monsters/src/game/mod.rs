use crate::renderer::texture;
use crate::renderer::State;
use crate::renderer::{spritebatch::Spritebatch, Vertex};
use std::sync::Arc;
use ultraviolet::Vec3;
use wgpu::{Device, Queue};

pub struct Scene {
    spritebatch: Spritebatch,
    grass_texture: Arc<texture::Texture>,
    ground_wall_texture: Arc<texture::Texture>,
}

impl Scene {
    pub fn new(device: Arc<Device>, queue: &Queue) -> Self {
        let spritebatch = Spritebatch::new(device.clone());

        let grass_bytes = include_bytes!("../res/grass.png");
        let grass_texture = texture::Texture::from_bytes(&device, &queue, grass_bytes, "grass.png");

        let ground_wall_bytes = include_bytes!("../res/ground-wall.png");
        let ground_wall_texture =
            texture::Texture::from_bytes(&device, &queue, ground_wall_bytes, "ground-wall.png");

        Self {
            spritebatch,
            grass_texture,
            ground_wall_texture,
        }
    }

    pub fn tick(&mut self, state: &mut State) {
        /*let (vertices, indices) = create_wall_verts(Vec3::new(-2., 0., 1.));
        self.spritebatch
            .push_verts(&vertices, &indices, self.ground_wall_texture.clone());

        let (vertices, indices) = create_wall_verts(Vec3::new(-1., 0., 1.));
        self.spritebatch
            .push_verts(&vertices, &indices, self.ground_wall_texture.clone());

        /*self.spritebatch.draw(
            Vec3::new(-2., 0., 1.),
            wall_angle,
            self.ground_wall_texture.clone(),
        );
        self.spritebatch.draw(
            Vec3::new(-1., 0., 1.),
            wall_angle,
            self.ground_wall_texture.clone(),
        );*/

        let (vertices, indices) = create_wall_verts(Vec3::new(-2., 1., -2.));
        self.spritebatch
            .push_verts(&vertices, &indices, self.ground_wall_texture.clone());

        let (vertices, indices) = create_wall_verts(Vec3::new(-1., 1., -2.));
        self.spritebatch
            .push_verts(&vertices, &indices, self.ground_wall_texture.clone());

        /*self.spritebatch.draw(
            Vec3::new(-2., 0.5_f32.sqrt(), -1. - (0.5_f32.sqrt())),
            wall_angle,
            self.ground_wall_texture.clone(),
        );
        self.spritebatch.draw(
            Vec3::new(-1., 0.5_f32.sqrt(), -1. - (0.5_f32.sqrt())),
            wall_angle,
            self.ground_wall_texture.clone(),
        );*/*/

        for column in -2..=2 {
            for row in -1..=1 {
                self.draw_floor(column, row, 0);
            }
        }

        for column in -2..=2 {
            self.draw_ground_wall(column, 2, 0);
            self.draw_ground_wall(column, -2, 1);
            self.draw_ground_wall(column, -3, 2);
        }

        for column in -5..=3 {
            for row in -5..=-4 {
                self.draw_floor(column, row, 2);
            }
        }

        for column in -5..=-3 {
            self.draw_ground_wall(column, -3, 2);
        }
        self.draw_ground_wall(3, -3, 2);

        for column in -5..=-3 {
            for row in -2..=1 {
                self.draw_floor(column, row, 1);
            }
        }

        let spritebatch_buffer = self.spritebatch.get_buffer();
        state.spritebatch_buffers = spritebatch_buffer;
    }

    fn draw_floor(&mut self, column: i32, row: i32, height: i32) {
        self.spritebatch.draw(
            Vec3::new(column as f32, height as f32, row as f32),
            Vec3::unit_y(),
            self.grass_texture.clone(),
        );
    }

    fn draw_ground_wall(&mut self, column: i32, row: i32, height: i32) {
        fn create_wall_verts(offset: Vec3) -> ([Vertex; 4], [u16; 6]) {
            let vertices: [Vertex; 4] = [
                Vertex {
                    position: (Vec3::new(0., 0., 0.) + offset).into(),
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: (Vec3::new(0., -1., 1.) + offset).into(),
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: (Vec3::new(1., -1., 1.) + offset).into(),
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: (Vec3::new(1., 0., 0.) + offset).into(),
                    tex_coords: [1.0, 0.0],
                },
            ];

            #[rustfmt::skip]
            let indices: [u16; 6] = [
                0, 1, 3,
                1, 2, 3,
            ];

            (vertices, indices)
        }

        let (vertices, indices) =
            create_wall_verts(Vec3::new(column as f32, height as f32, row as f32));
        self.spritebatch
            .push_verts(&vertices, &indices, self.ground_wall_texture.clone());
    }
}
