use crate::renderer::texture;
use crate::renderer::State;
use crate::renderer::{spritebatch::Spritebatch, Vertex};
use std::sync::Arc;
use wgpu::{Device, Queue};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Copy, Clone, Debug)]
pub enum Model {
    Wall,
    Corner,
    Floor,
}

impl Model {
    fn get_model(&self) -> (&[Vertex], &[u16]) {
        match &self {
            Model::Wall => (crate::models::wall::VERTS, crate::models::wall::INDICES),
            Model::Floor => (crate::models::floor::VERTS, crate::models::floor::INDICES),
            Model::Corner => (crate::models::corner::VERTS, crate::models::corner::INDICES),
        }
    }

    fn serialize(&self) -> &'static str {
        match &self {
            Model::Wall => "Model::Wall",
            Model::Corner => "Model::Corner",
            Model::Floor => "Model::Floor",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    height: u8,
    model: Model,
}

impl Tile {
    fn new(height: u8, model: Model) -> Self {
        Self { height, model }
    }
}

pub struct Scene {
    spritebatch: Spritebatch,
    grass_texture: Arc<texture::Texture>,
    ground_wall_texture: Arc<texture::Texture>,

    move_select_left: bool,
    move_select_right: bool,
    move_select_up: bool,
    move_select_down: bool,

    selected: (u8, u8),
    map: [[Tile; 16]; 12],
}

impl Scene {
    pub fn new(device: Arc<Device>, queue: &Queue) -> Self {
        let spritebatch = Spritebatch::new(device.clone());

        let grass_bytes = include_bytes!("../res/grass.png");
        let grass_texture = texture::Texture::from_bytes(&device, &queue, grass_bytes, "grass.png");

        let ground_wall_bytes = include_bytes!("../res/ground-wall.png");
        let ground_wall_texture =
            texture::Texture::from_bytes(&device, &queue, ground_wall_bytes, "ground-wall.png");

        let map = {
            [
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                    Tile::new(1, Model::Floor),
                ],
                [
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Wall),
                    Tile::new(0, Model::Corner),
                ],
            ]
        };

        Self {
            spritebatch,
            grass_texture,
            ground_wall_texture,

            move_select_down: false,
            move_select_left: false,
            move_select_right: false,
            move_select_up: false,

            selected: (0, 0),
            map,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Up => {
                        self.move_select_up = is_pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.move_select_left = is_pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.move_select_down = is_pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.move_select_right = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn serialize_map(&self) {
        let create_tile_str = |height: u8, model: Model| {
            format!("Tile::new({}, {}),", height.to_string(), model.serialize())
        };

        let create_row_str = |row: &[Tile]| {
            let mut row_string = "[".to_owned();
            for tile in row.iter() {
                let tile_str = create_tile_str(tile.height, tile.model);
                row_string.push_str(&tile_str);
            }
            row_string.push_str("],");

            row_string
        };

        let mut map_str = "let map = [".to_owned();
        for row in self.map.iter() {
            map_str.push_str(&create_row_str(row));
        }
        map_str.push_str("];");

        println!("{}", map_str);
    }

    pub fn tick(&mut self, state: &mut State) {
        if self.move_select_up && self.selected.1 > 0 {
            self.selected.1 -= 1;
        }
        if self.move_select_down && self.selected.1 < 11 {
            self.selected.1 += 1;
        }

        if self.move_select_left && self.selected.0 > 0 {
            self.selected.0 -= 1;
        }
        if self.move_select_right && self.selected.0 < 15 {
            self.selected.0 += 1;
        }

        self.serialize_map();

        for (y, row_data) in self.map.iter().enumerate() {
            for (x, tile) in row_data.iter().enumerate() {
                let (vertices, indices) = tile.model.get_model();
                let vertices: Vec<_> = vertices
                    .iter()
                    .map(|vertex| Vertex {
                        position: [
                            vertex.position[0] + x as f32,
                            vertex.position[1] + tile.height as f32 + {
                                if self.selected.0 as usize == x && self.selected.1 as usize == y {
                                    0.25
                                } else {
                                    0.0
                                }
                            },
                            vertex.position[2] + y as f32,
                        ],
                        tex_coords: vertex.tex_coords,
                    })
                    .collect();

                let texture = match &tile.model {
                    Model::Floor => self.grass_texture.clone(),
                    _ => self.ground_wall_texture.clone(),
                };

                self.spritebatch.push_verts(&vertices, indices, texture);
            }
        }

        let spritebatch_buffer = self.spritebatch.get_buffer();
        state.spritebatch_buffers = spritebatch_buffer;
    }
}
