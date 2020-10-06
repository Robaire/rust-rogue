extern crate nalgebra;
extern crate specs;

// Components
mod components {
    use specs::{Component, NullStorage, VecStorage};
    use crate::gl_util;

    /// Entity position in world coordinates
    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct Position {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    impl Position {
        pub fn new() -> Position {
            Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    /// Entity velocity in world coordinates
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Velocity {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    impl Velocity {
        pub fn new() -> Velocity {
            Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    /// Entity size on the tile
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Size {
        // Overall size in world coordinates
        pub width: f32,
        pub height: f32,
    }
    impl Size {
        pub fn default() -> Size {
            Size { width: 10.0, height: 10.0 }
        }
        pub fn new(width: f32, height: f32) -> Size {
            Size { width, height }
        }
    }

    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Drawn {
        pub program: u32,
        pub attribute_array: u32,
        pub vertex_buffer: u32,
        pub texture_id: u32,
        pub texture_coord_buffer: u32,
    }
    impl Drawn {
        pub fn new(
            program: u32,
            texture_id: u32,
            vertices: Vec<f32>,
            texture_vertices: Vec<f32>,
        ) -> Drawn {
            
            let attribute_array = gl_util::generate_buffer();
            
            let vertex_buffer = gl_util::generate_buffer();
            gl_util::set_buffer_data(vertex_buffer, vertices);

            let texture_coord_buffer = gl_util::generate_buffer();
            gl_util::set_buffer_data(texture_coord_buffer, texture_vertices);

            // TODO: Bind vertex buffers to the attribute array

            Drawn {
                program,
                attribute_array,
                vertex_buffer,
                texture_id,
                texture_coord_buffer,
            }
        }
    }

    /// Deteremines if an entity is animated
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Animate {
        pub speed: std::time::Duration,
        pub time_elapsed: std::time::Duration,
        pub texture_coord_buffer: u32,
        pub layer: usize,
        pub layer_coordinates: Vec<Vec<f32>>,
    }
    impl Animate {
        pub fn new(speed: f32, layer_coordinates: Vec<Vec<f32>>) -> Animate {

            // TODO: Layer_coordinates should probably be a vector of vertex buffers for quick switching

            Animate {
                speed: std::time::Duration::from_secs_f32(speed),
                time_elapsed: std::time::Duration::new(0, 0),
                texture_coord_buffer: 0,
                layer: 0,
                layer_coordinates,
            }
        }
    }

    /// If an entity is controlled
    #[derive(Component, Default)]
    #[storage(NullStorage)]
    pub struct Controlled;
}

/// ECS Resources
mod resources {

    /// Stores delta time
    pub struct DeltaTime {
        last: std::time::Instant,
        pub delta: std::time::Duration,
    }
    impl DeltaTime {
        pub fn update(&mut self) {
            let now = std::time::Instant::now();
            self.delta = now - self.last;
            self.last = now;
        }
    }
    impl Default for DeltaTime {
        fn default() -> DeltaTime {
            DeltaTime {
                last: std::time::Instant::now(),
                delta: std::time::Duration::new(0, 0),
            }
        }
    }

    /// Stores current keyboard inputs
    #[derive(Default, Debug)]
    pub struct InputState {
        pub up: bool,
        pub down: bool,
        pub left: bool,
        pub right: bool,
        pub action: bool,
    }
    impl InputState {
        pub fn new() -> InputState {
            InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false,
            }
        }
    }
}

/// Systems
mod systems {

    use super::components::*;
    use super::resources::*;
    use specs::prelude::*;

    /*
    /// Updates the texture coordinates of an entity
    pub struct AnimateSystem;
    impl<'a> System<'a> for AnimateSystem {
        type SystemData = (
            WriteStorage<'a, Animate>,
            WriteStorage<'a, Texture>,
            Read<'a, DeltaTime>,
        );

        fn run(&mut self, (mut animate, mut texture, delta_time): Self::SystemData) {
            // For every entity update its texture coordinates from its animation information
            for (animate, texture) in (&mut animate, &mut texture).join() {
                animate.time_elapsed += delta_time.delta;

                // Check if the next layer should be displayed
                if animate.time_elapsed >= animate.speed {
                    animate.layer += 1;

                    // Reset the layer count if necessary
                    if animate.layer >= animate.layer_coordinates.len() {
                        animate.layer = 0;
                    }

                    // Reset the elapsed time counter
                    animate.time_elapsed = std::time::Duration::new(0, 0);
                }

                // Update the texture coordinates
                texture.coordinates = animate.layer_coordinates[animate.layer];
            }
        }
    }
    */

    /*
    /// Draws entities to the screen
    pub struct DrawSystem;
    impl<'a> System<'a> for DrawSystem {
        type SystemData = (ReadStorage<'a, Draw>, ReadStorage<'a, Position>);

        fn run(&mut self, (draw, position): Self::SystemData) {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            };

            for (draw, position) in (&draw, &position).join() {
                let mut vertices = draw.vertices.clone();

                vertices[0] += position.x as f32;
                vertices[1] += position.y as f32;

                vertices[5] += position.x as f32;
                vertices[6] += position.y as f32;

                vertices[10] += position.x as f32;
                vertices[11] += position.y as f32;

                vertices[15] += position.x as f32;
                vertices[16] += position.y as f32;

                vertices[20] += position.x as f32;
                vertices[21] += position.y as f32;

                vertices[25] += position.x as f32;
                vertices[26] += position.y as f32;

                // Update Vertex Information
                unsafe {
                    gl::BindBuffer(gl::ARRAY_BUFFER, draw.vertex_buffer);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        vertices.as_ptr() as *const gl::types::GLvoid,
                        gl::STATIC_DRAW,
                    );
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                };

                match draw.draw_type {
                    DrawType::Static { texture_id } => {
                        unsafe {
                            gl::UseProgram(draw.shader_program);
                            gl::BindTexture(gl::TEXTURE_2D, texture_id);
                            gl::DrawArrays(gl::TRIANGLES, 0, 6);
                        };
                    }
                    DrawType::Dynamic { texture_id, layer } => unsafe {
                        gl::UseProgram(draw.shader_program);
                        gl::BindTexture(gl::TEXTURE_2D_ARRAY, texture_id);
                        gl::Uniform1i(
                            gl::GetUniformLocation(
                                draw.shader_program,
                                CString::new("layer").unwrap().as_ptr(),
                            ),
                            layer as i32,
                        );
                        gl::DrawArrays(gl::TRIANGLES, 0, 6);
                    },
                }
            }
        }
    }
    */

    /// Computes the delta time step
    pub struct TimeSystem;
    impl<'a> System<'a> for TimeSystem {
        type SystemData = Write<'a, DeltaTime>;

        fn run(&mut self, mut delta_time: Self::SystemData) {
            delta_time.update();
        }
    }

    /// Modifies entity velocity based on keyboard input
    pub struct ControlSystem;
    impl<'a> System<'a> for ControlSystem {
        type SystemData = (
            WriteStorage<'a, Velocity>,
            ReadStorage<'a, Controlled>,
            Read<'a, InputState>,
        );

        fn run(&mut self, (mut velocity, controlled, input_state): Self::SystemData) {
            let up = if input_state.up { 1.0 } else { 0.0 };
            let down = if input_state.down { -1.0 } else { 0.0 };

            let right = if input_state.right { 1.0 } else { 0.0 };
            let left = if input_state.left { -1.0 } else { 0.0 };

            for (velocity, _) in (&mut velocity, &controlled).join() {
                velocity.x = right + left;
                velocity.y = up + down;
            }
        }
    }

    /// Integrates position using velocity and delta time
    pub struct PhysicsSystem;
    impl<'a> System<'a> for PhysicsSystem {
        type SystemData = (
            WriteStorage<'a, Position>,
            ReadStorage<'a, Velocity>,
            Read<'a, DeltaTime>,
        );

        fn run(&mut self, (mut position, velocity, delta_time): Self::SystemData) {
            let delta = delta_time.delta.as_secs_f64();

            for (position, velocity) in (&mut position, &velocity).join() {
                position.x += velocity.x * delta;
                position.y += velocity.y * delta;
                position.z += velocity.z * delta;
            }
        }
    }
}
