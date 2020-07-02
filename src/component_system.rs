extern crate specs;
use specs::prelude::*;
use specs::{Component, VecStorage};

// Components
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Controlled;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Render {
    pub program_id: u32,
    pub texture_id: u32,
    pub vertex_buffer: u32,
    pub vertices: Vec<f32>
}

// Resources
pub struct DeltaTime {
    pub last: std::time::Instant,
    pub delta: std::time::Duration
}
impl Default for DeltaTime {
    fn default() -> DeltaTime {
        DeltaTime{ last: std::time::Instant::now(), delta: std::time::Duration::new(0, 0) }
    }
}

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

#[derive(Default)]
pub struct VertexInformation {
    pub vertices: Vec<f32>
}
impl VertexInformation {
    pub fn new(vertices: Vec<f32>) -> VertexInformation {
        VertexInformation { vertices }
    }
}

// Systems
pub struct RenderSystem;
impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Render>, ReadStorage<'a, Position>);

    fn run(&mut self, (render, position): Self::SystemData) {

        unsafe{
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        for (render, position) in (&render, &position).join() {

            let mut vertices = render.vertices.clone();

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


            // Prepare the GPU
            unsafe {

                gl::BindBuffer(gl::ARRAY_BUFFER, render.vertex_buffer);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);

                gl::UseProgram(render.program_id);
                gl::BindTexture(gl::TEXTURE_2D, render.texture_id);

                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            };

        }

    }
}

pub struct TimeSystem;
impl<'a> System<'a> for TimeSystem {
    type SystemData = Write<'a, DeltaTime>;

    fn run(&mut self, mut delta_time: Self::SystemData) {

        let now = std::time::Instant::now();
        delta_time.delta = now - delta_time.last;
        delta_time.last = now;
        
    }

}

pub struct ControlSystem;
impl<'a> System<'a> for ControlSystem {
    type SystemData = (WriteStorage<'a, Velocity>, ReadStorage<'a, Controlled>, Read<'a, InputState>);

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

pub struct PhysicsSystem;
impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>, Read<'a, DeltaTime>);

    fn run(&mut self, (mut position, velocity, delta_time): Self::SystemData) {

        let delta = delta_time.delta.as_secs_f64();

        for (position, velocity) in (&mut position, &velocity).join() {
            
            position.x += velocity.x * delta;
            position.y += velocity.y * delta;
            position.z += velocity.z * delta;
        }
    }
}