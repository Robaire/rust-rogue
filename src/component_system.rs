extern crate nalgebra;
extern crate specs;

// Components
pub mod components {
    use crate::gl_util;
    use specs::{Component, NullStorage, VecStorage};

    /// Entity position in world coordinates
    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct Position {
        pub x: f32,
        pub y: f32,
        pub z: f32
    }
    impl Position {
        pub fn new() -> Position {
            Position {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        }
        pub fn new_xyz(x: f32, y: f32, z: f32) -> Position {
            Position { x, y, z }
        }

        pub fn as_vec(&self) -> Vec<f32> {
            vec![self.x, self.y, self.z]
        }
    }

    /// Entity velocity in world coordinates
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Velocity {
        pub x: f32,
        pub y: f32,
        pub z: f32
    }
    impl Velocity {
        pub fn new() -> Velocity {
            Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        }

        pub fn as_vec(&self) -> Vec<f32> {
            vec![self.x, self.y, self.z]
        }
    }

    /// Entity size on the tile
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Size {
        // Overall size in world coordinates
        pub width: f32,
        pub height: f32
    }
    impl Size {
        pub fn default() -> Size {
            Size {
                width: 10.0,
                height: 10.0
            }
        }
        pub fn new(width: f32, height: f32) -> Size {
            Size { width, height }
        }

        pub fn as_vec(&self) -> Vec<f32> {
            vec![self.width, self.height]
        }
    }

    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Drawn {
        pub program: u32,
        pub attribute_array: u32,
        pub vertex_buffer: u32,
        pub vertex_count: u32,
        pub texture_id: u32,
        pub texture_coord_buffer: u32
    }
    impl Drawn {
        pub fn new(
            program: u32,
            texture_id: u32,
            vertices: Vec<f32>,
            texture_vertices: Vec<f32>
        ) -> Drawn {
            unsafe {
                gl::UseProgram(program);
            }

            let attribute_array = gl_util::generate_vertex_array();
            gl_util::bind_array(attribute_array);

            let vertex_buffer = gl_util::generate_buffer();
            gl_util::set_buffer_data(vertex_buffer, &vertices);
            gl_util::set_vertex_array_pointer(vertex_buffer, attribute_array, 0, 3);

            let texture_coord_buffer = gl_util::generate_buffer();
            gl_util::set_buffer_data(texture_coord_buffer, &texture_vertices);
            gl_util::set_vertex_array_pointer(texture_coord_buffer, attribute_array, 1, 2);

            Drawn {
                program,
                attribute_array,
                vertex_buffer,
                vertex_count: vertices.len() as u32,
                texture_id,
                texture_coord_buffer
            }
        }
    }

    /// Deteremines if an entity is animated
    #[derive(Component)]
    #[storage(VecStorage)]
    pub struct Animate {
        pub speed: std::time::Duration,
        pub time_elapsed: std::time::Duration,
        pub layer: u32,
        pub texture_coord_buffers: Vec<u32>
    }
    impl Animate {
        pub fn new(speed: f32, layer_coordinates: Vec<Vec<f32>>) -> Animate {
            let mut texture_coord_buffers = Vec::new();

            for vertices in layer_coordinates {
                let buffer = gl_util::generate_buffer();
                gl_util::set_buffer_data(buffer, &vertices);

                texture_coord_buffers.push(buffer);
            }

            Animate {
                speed: std::time::Duration::from_secs_f32(speed),
                time_elapsed: std::time::Duration::new(0, 0),
                layer: 0,
                texture_coord_buffers
            }
        }
    }

    /// If an entity is controlled
    #[derive(Component, Default)]
    #[storage(NullStorage)]
    pub struct Controlled;
}

/// ECS Resources
pub mod resources {

    /// Stores delta time
    pub struct DeltaTime {
        last: std::time::Instant,
        pub delta: std::time::Duration
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
                delta: std::time::Duration::new(0, 0)
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
        pub action: bool
    }
    impl InputState {
        pub fn new() -> InputState {
            InputState {
                up: false,
                down: false,
                left: false,
                right: false,
                action: false
            }
        }
    }
}

/// Systems
pub mod systems {

    use super::components::*;
    use super::resources::*;
    use crate::gl_util;
    use specs::prelude::*;

    /// Draws an entity to the screen
    pub struct DrawSystem;
    impl<'a> System<'a> for DrawSystem {
        type SystemData = (
            ReadStorage<'a, Drawn>,
            ReadStorage<'a, Position>,
            ReadStorage<'a, Size>
        );

        fn run(&mut self, (drawn, position, size): Self::SystemData) {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            };

            for (drawn, position, size) in (&drawn, &position, &size).join() {
                unsafe {
                    gl::UseProgram(drawn.program);
                }

                gl_util::bind_array(drawn.attribute_array);
                gl_util::bind_texture(drawn.texture_id);

                // Update layout information
                gl_util::set_uniform_float_vec3("position", drawn.program, &position.as_vec());
                gl_util::set_uniform_float_vec2("size", drawn.program, &size.as_vec());

                // Issue the draw call
                gl_util::draw_triangles(drawn.vertex_count / 3);
            }
        }
    }

    /// Updates texture coordinates
    pub struct AnimateSystem;
    impl<'a> System<'a> for AnimateSystem {
        type SystemData = (
            WriteStorage<'a, Animate>,
            ReadStorage<'a, Drawn>,
            Read<'a, DeltaTime>
        );

        fn run(&mut self, (mut animate, drawn, delta_time): Self::SystemData) {
            for (animate, drawn) in (&mut animate, &drawn).join() {
                animate.time_elapsed += delta_time.delta;

                if animate.time_elapsed >= animate.speed {
                    animate.time_elapsed = std::time::Duration::new(0, 0);
                    animate.layer += 1;

                    if animate.layer >= animate.texture_coord_buffers.len() as u32 {
                        animate.layer = 0;
                    }

                    gl_util::set_vertex_array_pointer(
                        animate.texture_coord_buffers[animate.layer as usize],
                        drawn.attribute_array,
                        1,
                        2
                    );
                }
            }
        }
    }

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
            Read<'a, InputState>
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
            Read<'a, DeltaTime>
        );

        fn run(&mut self, (mut position, velocity, delta_time): Self::SystemData) {
            let delta = delta_time.delta.as_secs_f32();

            for (position, velocity) in (&mut position, &velocity).join() {
                position.x += velocity.x * delta;
                position.y += velocity.y * delta;
                position.z += velocity.z * delta;
            }
        }
    }
}
