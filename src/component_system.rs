use std::ffi::CString;

extern crate specs;
use specs::prelude::*;
use specs::{Component, VecStorage};

// Components
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position { x: f64, y: f64, z: f64 }
impl Position {
    pub fn new() -> Position {
        Position{ x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity { x: f64, y: f64, z: f64 }
impl Velocity {
    pub fn new() -> Velocity {
        Velocity{ x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Animation {
    animation_speed: f32,
    time_elapsed: std::time::Duration,
    current_frame: u32,
    total_frames: u32
}
impl Animation {
    pub fn new(total_frames: u32) -> Animation {
        Animation { 
            animation_speed: 0.15, // Seconds
            time_elapsed: std::time::Duration::new(0, 0), 
            current_frame: 0, 
            total_frames 
        }
    }
}

pub enum DrawType {
    Static{ texture_id: u32 },
    Dynamic{ texture_id: u32, layer: u32 }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Draw {
    shader_program: u32,
    vertex_buffer: u32,
    vertices: Vec<f32>,
    draw_type: DrawType
}
impl Draw {
    pub fn new(shader_program: u32, vertex_buffer: u32, vertices: Vec<f32>, draw_type: DrawType) -> Draw {
        Draw {shader_program, vertex_buffer, vertices, draw_type }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Controlled;

// Resources
pub struct DeltaTime {
    last: std::time::Instant,
    delta: std::time::Duration
}
impl DeltaTime {
    fn update(&mut self) {
        let now = std::time::Instant::now();
        self.delta = now - self.last;
        self.last = now;
    }
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

// Systems

/// Updates the animation frame given the elapsed system time and the objects frame rate
pub struct AnimationSystem;
impl<'a> System<'a> for AnimationSystem {
    type SystemData = (WriteStorage<'a, Animation>, WriteStorage<'a, Draw>, Read<'a, DeltaTime>);

    fn run(&mut self, (mut animation, mut draw, delta_time): Self::SystemData) {

        for animation in (&mut animation).join() {
            
            animation.time_elapsed += delta_time.delta;

            // Check if the next frame should be displayed
            if animation.time_elapsed.as_secs_f32() >= animation.animation_speed {
                animation.current_frame += 1;
    
                // Set the frame back to 0 if the frame count exceeds its maximum
                if animation.current_frame >= animation.total_frames {
                    animation.current_frame = 0;
                }
    
                // Reset the elapsed time counter
                animation.time_elapsed = std::time::Duration::new(0, 0);
            }
        }

        // Update the layer for dynamically drawn entities
        for (animation, draw) in (&animation, &mut draw).join() {
            match draw.draw_type {
                DrawType::Dynamic{texture_id, layer} => draw.draw_type = DrawType::Dynamic{texture_id, layer: animation.current_frame},
                _ => ()
            }
        }
    }
}

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
                    gl::STATIC_DRAW
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            };

            match draw.draw_type {
                DrawType::Static{texture_id} => {
                    unsafe {
                        gl::UseProgram(draw.shader_program);
                        gl::BindTexture(gl::TEXTURE_2D, texture_id);
                        gl::DrawArrays(gl::TRIANGLES, 0, 6);
                    };
                },
                DrawType::Dynamic{texture_id, layer} => {
                    unsafe {
                        gl::UseProgram(draw.shader_program);
                        gl::BindTexture(gl::TEXTURE_2D_ARRAY, texture_id);
                        gl::Uniform1i(gl::GetUniformLocation(draw.shader_program, CString::new("layer").unwrap().as_ptr()), layer as i32);
                        gl::DrawArrays(gl::TRIANGLES, 0, 6);
                    }
                }
            }
        }
    }
}

pub struct TimeSystem;
impl<'a> System<'a> for TimeSystem {
    type SystemData = Write<'a, DeltaTime>;

    fn run(&mut self, mut delta_time: Self::SystemData) {

        delta_time.update();
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