macro_rules! offset {
    [$offset: ident, $a: literal, $b: literal, $c: literal] => {
        [$a - $offset[0], $b - $offset[1], $c - $offset[2]]
    };
}

pub mod wall {
    use crate::renderer::Vertex;

    //0  4
    //1  5
    //2  6
    //3  7

    pub const ORIGIN: [f32; 3] = [0.5, 0.5, 0.5];

    #[rustfmt::skip]
    pub const VERTS: &[Vertex] = &[
        Vertex { position: offset![ORIGIN, 0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: offset![ORIGIN, 0.0, 0.875, 0.25], tex_coords: [0.0, 0.125] },
        Vertex { position: offset![ORIGIN, 0.0, 0.125, 0.75], tex_coords: [0.0, 0.875] },
        Vertex { position: offset![ORIGIN, 0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]},
        
        Vertex { position: offset![ORIGIN, 1.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: offset![ORIGIN, 1.0, 0.875, 0.25], tex_coords: [1.0, 0.125] },
        Vertex { position: offset![ORIGIN, 1.0, 0.125, 0.75], tex_coords: [1.0, 0.875] },
        Vertex { position: offset![ORIGIN, 1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ];

    #[rustfmt::skip]
    pub const INDICES: &[u16] = &[
        0, 1, 4,
        4, 1, 5,
        
        1, 2, 5,
        5, 2, 6,
        
        2, 3, 6,
        6, 3, 7,
    ];
}

pub mod corner {
    use crate::renderer::Vertex;

    pub const ORIGIN: [f32; 3] = [0.5, 0.5, 0.5];

    #[rustfmt::skip]
    pub const VERTS: &[Vertex] = &[
        // TOP
        Vertex { position: offset![ORIGIN, 0.0, 1.0, 0.0], tex_coords: [0.5, 0.0] }, // 0

        // LEFT SIDE
        Vertex { position: offset![ORIGIN, 0.0, 0.875, 0.25], tex_coords: [0.375, 0.125] }, // 1
        Vertex { position: offset![ORIGIN, 0.0, 0.125, 0.75], tex_coords: [0.125, 0.875] }, // 2
        Vertex { position: offset![ORIGIN, 0.0, 0.0, 1.0], tex_coords: [0.0, 1.0] }, // 3

        // BACK SIDE
        Vertex { position: offset![ORIGIN, 0.25, 0.875, 0.0], tex_coords: [0.625, 0.125] }, // 4
        Vertex { position: offset![ORIGIN, 0.75, 0.125, 0.0], tex_coords: [0.875, 0.875] }, // 5
        Vertex { position: offset![ORIGIN, 1.0, 0.0, 0.0], tex_coords: [1.0, 1.0] }, // 6

        // CORNER TOP
        Vertex { position: offset![ORIGIN, 0.5, 0.125, 0.625], tex_coords: [0.4, 0.875] }, // 7
        Vertex { position: offset![ORIGIN, 0.625, 0.125, 0.5], tex_coords: [0.6, 0.875] }, // 8

        // CORNER BOTTOM
        Vertex { position: offset![ORIGIN, 0.5, 0.0, 0.875], tex_coords: [0.35, 1.0] }, // 9
        Vertex { position: offset![ORIGIN, 0.875, 0.0, 0.5], tex_coords: [0.65, 1.0] }, // 10
        
    ];

    #[rustfmt::skip]
    pub const INDICES: &[u16] = &[
        // Top
        0, 1, 4,

        // Left
        1, 2, 7,

        // Back
        4, 8, 5,

        // Center
        1, 7, 4,
        4, 7, 8,

        // Corner Left
        2, 3, 7,
        7, 3, 9,

        // Corner Center
        7, 9, 8,
        8, 9, 10,

        // Corner Back
        8, 10, 5,
        5, 10, 6,
    ];
}

pub mod floor {
    use crate::renderer::Vertex;

    pub const ORIGIN: [f32; 3] = [0.5, 0.5, 0.5];

    #[rustfmt::skip]
    pub const VERTS: &[Vertex] = &[
        Vertex { position: offset![ORIGIN, 0.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: offset![ORIGIN, 0.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: offset![ORIGIN, 1.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: offset![ORIGIN, 1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ];

    #[rustfmt::skip]
    pub const INDICES: &[u16] = &[
        0, 1, 2,
        2, 1, 3,
    ];
}
