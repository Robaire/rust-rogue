use std::collections::HashMap;

pub fn create_map_entities(world: &mut specs::World, program: u32) {
    use crate::component_system::components::*;
    use crate::gl_util;
    use crate::specs::*;

    let vertices = create_rectangle(16.0, 16.0);

    let floor_texture_vertices: Vec<f32> =
        create_texture_coordinates(16.0 / 112.0, 16.0 / 208.0, 16.0 / 112.0, 128.0 / 208.0);

    let texture_id = gl_util::create_texture_from_file("./src/textures/map_tiles.png");

    world
        .create_entity()
        .with(Position::new_xyz(0.0, 0.0, 0.0))
        .with(Size::new(0.3, 0.3))
        .with(Drawn::new(
            program,
            texture_id,
            vertices.clone(),
            floor_texture_vertices.clone()
        ))
        .build();
    world
        .create_entity()
        .with(Position::new_xyz(0.3, 0.0, 0.0))
        .with(Size::new(0.3, 0.3))
        .with(Drawn::new(
            program,
            texture_id,
            vertices.clone(),
            floor_texture_vertices.clone()
        ))
        .build();
    world
        .create_entity()
        .with(Position::new_xyz(0.0, 0.3, 0.0))
        .with(Size::new(0.3, 0.3))
        .with(Drawn::new(
            program,
            texture_id,
            vertices.clone(),
            floor_texture_vertices.clone()
        ))
        .build();

    world
        .create_entity()
        .with(Position::new_xyz(0.3, 0.3, 0.0))
        .with(Size::new(0.3, 0.3))
        .with(Drawn::new(
            program,
            texture_id,
            vertices.clone(),
            floor_texture_vertices.clone()
        ))
        .build();
}

/// Creates texture coordinates
/// # Arguments
/// * `width` - Texture width
/// * `height` - Texture height
/// * `x` - Position of texture origin
/// * 'y' - Position of texture origin
fn create_texture_coordinates(width: f32, height: f32, x: f32, y: f32) -> Vec<f32> {
    vec![
        x,
        y,
        x + width,
        y + height,
        x,
        y + height,
        x,
        y,
        x + width,
        y,
        x + width,
        y + height,
    ]
}

/// Creates a rectangle normalized to (-1, 1)
/// # Arguments
/// * `width` - Rectangle width
/// * `height` - Rectangle height
fn create_rectangle(width: f32, height: f32) -> Vec<f32> {
    let aspect_ratio = width / height;

    // Start with the vertices arranged as a square
    let mut vertices: Vec<f32> = vec![
        -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0,
        0.0,
    ];

    if width > height {
        // Divide y by aspect ratio
        vertices[1] /= aspect_ratio;
        vertices[4] /= aspect_ratio;
        vertices[7] /= aspect_ratio;
        vertices[10] /= aspect_ratio;
        vertices[13] /= aspect_ratio;
        vertices[16] /= aspect_ratio;
    } else if height > width {
        // Multiply x by aspect ratio
        vertices[0] *= aspect_ratio;
        vertices[3] *= aspect_ratio;
        vertices[6] *= aspect_ratio;
        vertices[9] *= aspect_ratio;
        vertices[12] *= aspect_ratio;
        vertices[15] *= aspect_ratio;
    }

    return vertices;
}

pub fn read_map_string() {
    let map = "
    #########
    #.......#
    #.......#
    #.......#
    #########
    ";

}

#[derive(PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

enum TileType {
    Floor,
    Wall
}

struct Tile {
    position: Coordinate,
    tile_type: TileType
}

struct Map {
    tiles: HashMap<Coordinate, Tile>
}

impl Map {
    fn add_tile(&mut self, coordinate: Coordinate, tile: Tile) {
        self.tiles.insert(coordinate, tile);
    }

    pub fn create_map(template: &str) -> Map {
        let mut map = Map{ tiles: HashMap::new() };

        


        return map;
    }
}