use crate::renderer::texture;
use crate::renderer::State;
use crate::renderer::{spritebatch::Spritebatch, Vertex};
use std::sync::Arc;
use wgpu::{Device, Queue};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

mod map;

#[derive(Copy, Clone, Debug)]
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

    fn serialize(&self) -> &'static str {
        match &self {
            Model::Wall => "Model::Wall",
            Model::Corner => "Model::Corner",
            Model::Floor => "Model::Floor",
            Model::InnerCorner => "Model::InnerCorner",
        }
    }
}

/// Counter clockwise
#[derive(Copy, Clone, Debug)]
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

    fn serialize(&self) -> &'static str {
        match &self {
            TileRotation::Zero => "TileRotation::Zero",
            TileRotation::Quarter => "TileRotation::Quarter",
            TileRotation::Half => "TileRotation::Half",
            TileRotation::ThreeQuarters => "TileRotation::ThreeQuarters",
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

        let map = map::map();

        Self {
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

    fn serialize_map(&self) {
        let create_tile_str = |height: u8, model: Model, rotation: TileRotation| {
            format!(
                "Tile::new_rotation({}, {}, {}),",
                height.to_string(),
                model.serialize(),
                rotation.serialize()
            )
        };

        let create_row_str = |row: &[Tile]| {
            let mut row_string = "[".to_owned();
            for tile in row.iter() {
                let tile_str = create_tile_str(tile.height, tile.model, tile.rotation);
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
        self.move_select_up = false;
        self.move_select_down = false;
        self.move_select_left = false;
        self.move_select_right = false;

        if self.save_map_return {
            self.serialize_map();
        }
        self.save_map_return = false;

        if self.raise {
            let tile = &mut (&mut self.map[self.selected.1 as usize])[self.selected.0 as usize];
            if tile.height != u8::MAX {
                tile.height += 1;
            }
        }

        if self.lower {
            let tile = &mut (&mut self.map[self.selected.1 as usize])[self.selected.0 as usize];
            if tile.height > 0 {
                tile.height -= 1;
            }
        }

        self.raise = false;
        self.lower = false;

        if self.swap_model {
            let tile = &mut (&mut self.map[self.selected.1 as usize])[self.selected.0 as usize];
            tile.model.swap_model();
        }
        self.swap_model = false;

        if self.rotate_tile {
            let tile = &mut (&mut self.map[self.selected.1 as usize])[self.selected.0 as usize];
            tile.rotation.rotate_next();
        }
        self.rotate_tile = false;

        fn produce_verts(
            tile: &Tile,
            (x, y): (usize, usize),
            rotation: TileRotation,
            selected: (u8, u8),
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
                            if selected.0 as usize == x && selected.1 as usize == y {
                                0.25
                            } else {
                                0.0
                            }
                        },
                        vertex.position[2] + y as f32,
                    ],
                    tex_coords: vertex.tex_coords,
                })
                /*.map(|vertex| Vertex {
                    position: [
                        vertex.position[0],
                        vertex.position[1],
                        vertex.position[2] * 1.25,
                    ],
                    tex_coords: vertex.tex_coords,
                })*/
                .collect();
            (vertices, indices)
        }

        for (y, row_data) in self.map.iter().enumerate() {
            for (x, tile) in row_data.iter().enumerate().filter(|(_, tile)| {
                if let Model::Floor = tile.model {
                    true
                } else {
                    false
                }
            }) {
                let (vertices, indices) =
                    produce_verts(tile, (x, y), tile.rotation, self.selected, Model::Floor);
                let texture = self.grass_texture.clone();

                self.spritebatch.push_verts(&vertices, indices, texture);
            }
        }

        for (y, row_data) in self.map.iter().enumerate() {
            for (x, tile) in row_data.iter().enumerate().filter(|(_, tile)| {
                if let Model::Floor = tile.model {
                    false
                } else {
                    true
                }
            }) {
                let (vertices, indices) =
                    produce_verts(tile, (x, y), tile.rotation, self.selected, tile.model);

                let texture = self.ground_wall_texture.clone();

                self.spritebatch.push_verts(&vertices, indices, texture);

                if let Model::Corner = tile.model {
                    let (vertices, indices) = produce_verts(
                        tile,
                        (x, y),
                        TileRotation::Zero,
                        self.selected,
                        Model::Floor,
                    );
                    let texture = self.grass_texture.clone();
                    self.spritebatch.push_verts(&vertices, &indices, texture);
                }
            }
        }

        let spritebatch_buffer = self.spritebatch.get_buffer();
        state.spritebatch_buffers = spritebatch_buffer;
    }
}
