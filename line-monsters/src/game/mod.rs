use crate::renderer::texture;
use crate::renderer::State;
use crate::renderer::{spritebatch::Spritebatch, Vertex};
use nanoserde::{DeBin, SerBin};
use std::sync::Arc;
use wgpu::{Device, Queue};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(DeBin, SerBin, Copy, Clone, Debug)]
pub enum Model {
    Wall,
    Corner,
    InnerCorner,
    Floor,
}

impl Model {
    fn get_model(&self) -> (&'static [Vertex], &'static [u16]) {
        match &self {
            Model::Wall => (crate::models::wall::VERTS, crate::models::wall::INDICES),
            Model::Floor => (crate::models::floor::VERTS, crate::models::floor::INDICES),
            Model::Corner => (crate::models::corner::VERTS, crate::models::corner::INDICES),
            Model::InnerCorner => (
                crate::models::inner_corner::VERTS,
                crate::models::inner_corner::INDICES,
            ),
        }
    }

    fn swap_model(&mut self) {
        match self {
            Model::Wall => *self = Model::Floor,
            Model::Floor => *self = Model::Corner,
            Model::Corner => *self = Model::InnerCorner,
            Model::InnerCorner => *self = Model::Wall,
        }
    }
}

/// Counter clockwise
#[derive(DeBin, SerBin, Copy, Clone, Debug)]
pub enum TileRotation {
    Zero,
    Quarter,
    Half,
    ThreeQuarters,
}

impl TileRotation {
    fn rotate_vertice(&self, vertice: &Vertex) -> Vertex {
        let rotate_90 = |vertex: Vertex| Vertex {
            position: [vertex.position[2], vertex.position[1], -vertex.position[0]],
            tex_coords: vertex.tex_coords,
        };

        match self {
            TileRotation::Zero => *vertice,
            TileRotation::Quarter => rotate_90(*vertice),
            TileRotation::Half => rotate_90(rotate_90(*vertice)),
            TileRotation::ThreeQuarters => rotate_90(rotate_90(rotate_90(*vertice))),
        }
    }

    fn rotate_next(&mut self) {
        match self {
            TileRotation::Zero => *self = TileRotation::Quarter,
            TileRotation::Quarter => *self = TileRotation::Half,
            TileRotation::Half => *self = TileRotation::ThreeQuarters,
            TileRotation::ThreeQuarters => *self = TileRotation::Zero,
        }
    }
}

#[derive(DeBin, SerBin, Copy, Clone, Debug)]
pub struct Tile {
    height: u8,
    model: Model,
    rotation: TileRotation,
}

impl Tile {
    fn new_rotation(height: u8, model: Model, rotation: TileRotation) -> Self {
        Self {
            height,
            model,
            rotation,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub x: isize,
    pub y: isize,
    pub tiles: [Tile; Self::WIDTH * Self::HEIGHT],
}

impl DeBin for Chunk {
    fn de_bin(o: &mut usize, d: &[u8]) -> std::result::Result<Self, nanoserde::DeBinErr> {
        std::result::Result::Ok(Self {
            x: DeBin::de_bin(o, d)?,
            y: DeBin::de_bin(o, d)?,
            tiles: DeBin::de_bin(o, d)?,
        })
    }
}
impl SerBin for Chunk {
    fn ser_bin(&self, s: &mut Vec<u8>) {
        self.x.ser_bin(s);
        self.y.ser_bin(s);
        self.tiles.ser_bin(s);
    }
}

impl Chunk {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;

    const I_WIDTH: isize = 32;
    const I_HEIGHT: isize = 32;

    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            tiles: [Tile::new_rotation(0, Model::Floor, TileRotation::Zero);
                Self::WIDTH * Self::HEIGHT],
        }
    }
}

#[derive(DeBin, SerBin)]
pub struct Map(Vec<Chunk>);

pub struct Scene {
    spritebatch: Spritebatch,
    grass_texture: Arc<texture::Texture>,
    ground_wall_texture: Arc<texture::Texture>,

    move_select_left: bool,
    move_select_right: bool,
    move_select_up: bool,
    move_select_down: bool,

    save_map_return: bool,

    raise: bool,
    lower: bool,

    swap_model: bool,

    rotate_tile: bool,

    selected: (isize, isize),

    chunks: Map,
}

impl Scene {
    pub fn new(device: Arc<Device>, queue: &Queue) -> Self {
        let spritebatch = Spritebatch::new(device.clone());

        let grass_bytes = include_bytes!("../res/grass.png");
        let grass_texture = texture::Texture::from_bytes(&device, &queue, grass_bytes, "grass.png");

        let ground_wall_bytes = include_bytes!("../res/ground-wall.png");
        let ground_wall_texture =
            texture::Texture::from_bytes(&device, &queue, ground_wall_bytes, "ground-wall.png");

        let chunks = Vec::new();

        let mut scene = Self {
            spritebatch,
            grass_texture,
            ground_wall_texture,

            move_select_down: false,
            move_select_left: false,
            move_select_right: false,
            move_select_up: false,

            save_map_return: false,

            raise: false,
            lower: false,

            swap_model: false,

            rotate_tile: false,

            selected: (0, 0),

            chunks: Map(chunks),
        };

        scene.deserialize();

        scene
    }

    pub fn serialize(&self) {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let bytes = SerBin::serialize_bin(&self.chunks);

        let path = Path::new("map_data.blob");

        let mut file = File::create(&path).unwrap();

        file.write_all(&bytes).unwrap();
    }

    pub fn deserialize(&mut self) {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let path = Path::new("map_data.blob");

        let mut file = File::open(&path).unwrap();

        let mut data = Vec::with_capacity(std::mem::size_of::<Chunk>());
        file.read_to_end(&mut data).unwrap();

        let map: Map = DeBin::deserialize_bin(&data).unwrap();
        self.chunks = map;
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
                    VirtualKeyCode::Return => {
                        self.save_map_return = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.raise = is_pressed;
                        true
                    }
                    VirtualKeyCode::C => {
                        self.lower = is_pressed;
                        true
                    }
                    VirtualKeyCode::X => {
                        self.swap_model = is_pressed;
                        true
                    }
                    VirtualKeyCode::R => {
                        self.rotate_tile = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn xy_to_chunk_coord(x: isize, y: isize) -> (isize, isize, usize, usize) {
        let chunk_x = f32::floor(x as f32 / 32.0) as isize;
        let chunk_y = f32::floor(y as f32 / 32.0) as isize;

        let (width, height) = (Chunk::I_WIDTH, Chunk::I_HEIGHT);
        let x_pos = if x >= 0 {
            x % width
        } else {
            width - 1 - isize::abs((x + 1) % width)
        };
        let y_pos = if y >= 0 {
            y % height
        } else {
            height - 1 - isize::abs((y + 1) % height)
        };

        assert!(x_pos >= 0 && y_pos >= 0);

        (chunk_x, chunk_y, x_pos as usize, y_pos as usize)
    }

    pub fn get_tile(&mut self, x: isize, y: isize) -> Option<&mut Tile> {
        let (chunk_x, chunk_y, x, y) = Self::xy_to_chunk_coord(x, y);

        let chunk = self
            .chunks
            .0
            .iter_mut()
            .find(|chunk| chunk.x == chunk_x && chunk.y == chunk_y)?;

        Some(&mut chunk.tiles[(y * 32 + x) as usize])
    }

    pub fn tick(&mut self, state: &mut State) {
        if self.move_select_up {
            self.selected.1 -= 1;
        }
        if self.move_select_down {
            self.selected.1 += 1;
        }

        if self.move_select_left {
            self.selected.0 -= 1;
        }
        if self.move_select_right {
            self.selected.0 += 1;
        }
        self.move_select_up = false;
        self.move_select_down = false;
        self.move_select_left = false;
        self.move_select_right = false;

        if self.save_map_return {
            self.serialize();
        }
        self.save_map_return = false;

        if self.raise {
            if let Some(tile) = self.get_tile(self.selected.0, self.selected.1) {
                if tile.height != u8::MAX {
                    tile.height += 1;
                }
            }
        }

        if self.lower {
            if let Some(tile) = self.get_tile(self.selected.0, self.selected.1) {
                if tile.height > 0 {
                    tile.height -= 1;
                }
            }
        }

        self.raise = false;
        self.lower = false;

        if self.swap_model {
            if let Some(tile) = self.get_tile(self.selected.0, self.selected.1) {
                tile.model.swap_model();
            }
        }
        self.swap_model = false;

        if self.rotate_tile {
            if let Some(tile) = self.get_tile(self.selected.0, self.selected.1) {
                tile.rotation.rotate_next();
            }
        }
        self.rotate_tile = false;

        fn produce_verts(
            tile: &Tile,
            (x, y): (isize, isize),
            rotation: TileRotation,
            (selected_x, selected_y): (isize, isize),
            model: Model,
        ) -> (Vec<Vertex>, &'static [u16]) {
            let (vertices, indices) = model.get_model();
            let vertices: Vec<_> = vertices
                .iter()
                .map(|vertex| rotation.rotate_vertice(vertex))
                .map(|vertex| Vertex {
                    position: [
                        vertex.position[0] + x as f32,
                        vertex.position[1] + tile.height as f32 + {
                            if selected_x == x && selected_y == y {
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
            (vertices, indices)
        }

        for chunk in self.chunks.0.iter() {
            let map = &chunk.tiles;

            for (x, y, tile) in map
                .iter()
                .enumerate()
                .filter(|(_, tile)| {
                    if let Model::Floor = tile.model {
                        true
                    } else {
                        if let Model::Corner = tile.model {
                            true
                        } else {
                            false
                        }
                    }
                })
                .map(|(n, tile)| {
                    (
                        (n as isize % Chunk::I_WIDTH) + chunk.x * Chunk::I_WIDTH, // x
                        (n as isize / Chunk::I_HEIGHT) + chunk.y * Chunk::I_HEIGHT, // y
                        tile,
                    )
                })
            {
                let (vertices, indices) =
                    produce_verts(tile, (x, y), tile.rotation, self.selected, Model::Floor);
                let texture = self.grass_texture.clone();

                self.spritebatch.push_verts(&vertices, indices, texture);
            }

            for (x, y, tile) in map
                .iter()
                .enumerate()
                .filter(|(_, tile)| {
                    if let Model::Floor = tile.model {
                        false
                    } else {
                        true
                    }
                })
                .map(|(n, tile)| {
                    (
                        (n as isize % Chunk::I_WIDTH) + chunk.x * Chunk::I_WIDTH, // x
                        (n as isize / Chunk::I_HEIGHT) + chunk.y * Chunk::I_HEIGHT, // y
                        tile,
                    )
                })
            {
                let (vertices, indices) =
                    produce_verts(tile, (x, y), tile.rotation, self.selected, tile.model);

                let texture = self.ground_wall_texture.clone();

                self.spritebatch.push_verts(&vertices, indices, texture);
            }
        }

        let spritebatch_buffer = self.spritebatch.get_buffer();
        state.spritebatch_buffers = spritebatch_buffer;
    }
}
