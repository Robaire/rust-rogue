extern crate gl;
extern crate sdl2;
use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

extern crate image;

pub mod shader;
use shader::{Shader, Program};

extern crate specs;
use specs::prelude::*;
use specs::{Component, VecStorage};
use specs::WorldExt;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position { x: f64, y: f64, z: f64 }

#[derive(Component)]
#[storage(VecStorage)]
struct Velocity { x: f64, y: f64, z: f64 }

#[derive(Component, Default)]
#[storage(NullStorage)]
struct Controlled;

#[derive(Component)]
#[storage(VecStorage)]
struct Render {
    program_id: u32,
    texture_id: u32,
    vertex_buffer: u32,
    vertices: Vec<f32>
}

struct RenderSystem;
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

            // println!("{}, {}, {}", render.vertex_buffer, render.program_id, render.texture_id);

            // Prepare the GPU
            unsafe {

                //gl::BindBuffer(gl::ARRAY_BUFFER, render.vertex_buffer as GL);
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

struct TimeSystem;
impl<'a> System<'a> for TimeSystem {
    type SystemData = Write<'a, DeltaTime>;

    fn run(&mut self, mut delta_time: Self::SystemData) {

        let now = std::time::Instant::now();
        delta_time.delta = now - delta_time.last;
        delta_time.last = now;
        
    }

}

struct ControlSystem;
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

struct PhysicsSystem;
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

struct PositionUpdateSystem;
impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (ReadStorage<'a, Position>, Write<'a, VertexInformation>);

    fn run(&mut self, (position, mut vi): Self::SystemData) {

        let vertices: Vec<f32> = vec![
            -0.3, -0.3, 0.0, 0.0, 0.0,
            0.3, 0.3, 0.0, 1.0, 1.0,
            -0.3, 0.3, 0.0, 0.0, 1.0,
            -0.3, -0.3, 0.0, 0.0, 0.0,
            0.3, -0.3, 0.0, 1.0, 0.0,
            0.3, 0.3, 0.0, 1.0, 1.0
        ];

        for position in (&position).join() {

            vi.vertices[0] = vertices[0] + position.x as f32;
            vi.vertices[5] = vertices[5] + position.x as f32;
            vi.vertices[10] = vertices[10] + position.x as f32;
            vi.vertices[15] = vertices[15] + position.x as f32;
            vi.vertices[20] = vertices[20] + position.x as f32;
            vi.vertices[25] = vertices[25] + position.x as f32;

            vi.vertices[1] = vertices[1] + position.y as f32;
            vi.vertices[6] = vertices[6] + position.y as f32;
            vi.vertices[11] = vertices[11] + position.y as f32;
            vi.vertices[16] = vertices[16] + position.y as f32;
            vi.vertices[21] = vertices[21] + position.y as f32;
            vi.vertices[26] = vertices[26] + position.y as f32;

            // println!("Position: {:?}", position);
        }
    }
}

struct DeltaTime {
    last: std::time::Instant,
    delta: std::time::Duration
}
impl Default for DeltaTime {
    fn default() -> DeltaTime {
        DeltaTime{ last: std::time::Instant::now(), delta: std::time::Duration::new(0, 0) }
    }
}

#[derive(Default, Debug)]
struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action: bool
}
impl InputState {
    fn new() -> InputState {
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
struct VertexInformation {
    pub vertices: Vec<f32>
}
impl VertexInformation {
    fn new(vertices: Vec<f32>) -> VertexInformation {
        VertexInformation { vertices }
    }
}

fn main() {

    // ECS Stuff
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Controlled>();
    world.register::<Render>();
    world.insert(DeltaTime{
        last: std::time::Instant::now(),
        delta: std::time::Duration::from_secs(0)
    });
    world.insert(InputState::new());
    world.insert(VertexInformation::new(vec![]));

    let mut dispatcher = DispatcherBuilder::new()
    .with(TimeSystem, "TimeSystem", &[])
    .with(ControlSystem, "ControlSystem", &["TimeSystem"])
    .with(PhysicsSystem, "PhysicsSystem", &["ControlSystem"])
    // .with(PositionUpdateSystem, "PositionUpdater", &["PhysicsSystem"])
    // .with(RenderSystem, "RenderSystem", &["PhysicsSystem"])
    .with_thread_local(RenderSystem)
    .build();
    
    // Initialize SDL
    let sdl_context = match sdl2::init() {
        Ok(context) => context,
        Err(message) => panic!(format!("SDL Init Failed: {}", message))
    };

    // Ask SDL to initialize the vide system
    let video_subsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(message) => panic!(format!("Failed to create video subsystem: {}", message))
    };

    // Set the attributes of the OpenGL Context
    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(GLProfile::Core);
    gl_attributes.set_context_flags().debug().set();
    gl_attributes.set_context_version(3, 3);
    // gl_attributes.set_multisample_buffers(1);
    // gl_attributes.set_multisample_samples(4);

    // Create the window
    let window = match video_subsystem
        .window("Rust Rouge", 600, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(message) => panic!(format!("Failed to create window: {}", message))
        };

    assert_eq!(gl_attributes.context_profile(), GLProfile::Core);
    assert_eq!(gl_attributes.context_version(), (3, 3));

    // Create the OpenGL Context
    let gl_context = match window.gl_create_context() {
        Ok(context) => context,
        Err(message) => panic!(format!("Failed to create OpenGL Context: {}", message))
    };

    // Load the OpenGL Functions
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    // Load shaders
    let vertex_shader = match Shader::new_from_file("./src/shaders/vertex.vert", gl::VERTEX_SHADER) {
        Ok(shader) => shader,
        Err(message) => panic!(format!("Failed to create vertex shader: {}", message))
    };

    let fragment_shader = match Shader::new_from_file("./src/shaders/fragment.frag", gl::FRAGMENT_SHADER) {
        Ok(shader) => shader,
        Err(message) => panic!(format!("Failed to create fragment shader: {}", message))
    };

    // Create shader program
    let shader_program = match Program::new().attach_shader(&vertex_shader).attach_shader(&fragment_shader).link() {
        Ok(program) => program,
        Err(message) => panic!(format!("Failed to create shader program: {}", message))
    };

    // Use shader program
    shader_program.set_used();

    // Create Square
    let square_vertices: Vec<f32> = vec![
        -0.3, -0.3, 0.0, 0.0, 0.0,
        0.3, 0.3, 0.0, 1.0, 1.0,
        -0.3, 0.3, 0.0, 0.0, 1.0,
        -0.3, -0.3, 0.0, 0.0, 0.0,
        0.3, -0.3, 0.0, 1.0, 0.0,
        0.3, 0.3, 0.0, 1.0, 1.0
    ];
    
    // Create Square
    let small_square_vertices: Vec<f32> = vec![
        -0.1, -0.1, 0.0, 0.0, 0.0,
        0.1, 0.1, 0.0, 1.0, 1.0,
        -0.1, 0.1, 0.0, 0.0, 1.0,
        -0.1, -0.1, 0.0, 0.0, 0.0,
        0.1, -0.1, 0.0, 1.0, 0.0,
        0.1, 0.1, 0.0, 1.0, 1.0
    ];

    {
    let mut vi = world.write_resource::<VertexInformation>();
    vi.vertices = square_vertices.to_vec();
    }

    // Create a vertex buffer
    let mut square_vbo = 0;
    unsafe { gl::GenBuffers(1, &mut square_vbo); };

    // Copy Vertex Data into the buffer
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, square_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (square_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            square_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    };

    // Create the vertex array and bind pointers to buffer information
    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); };
    unsafe { 
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, square_vbo);

        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            0, 3, gl::FLOAT, gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );

        let offset = 3;
        gl::VertexAttribPointer(
            1, 2, gl::FLOAT, gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (offset * std::mem::size_of::<f32>()) as *const std::ffi::c_void
        );
    };

    // Load the Image
    // TODO: Fix transparency values
    let image = match image::open("./src/frames/ogre_idle_anim_f0.png") {
        Ok(image) => image.flipv().into_rgba(),
        Err(message) => panic!(format!("Image could not be loaded: {}", message))
    };
    
    // Create a texture
    let mut texture_id = 0;
    unsafe { gl::GenTextures(1, &mut texture_id); };
    
    // Give the texture image data
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGB as i32, 
            image.width() as i32, 
            image.height() as i32, 
            0, gl::RGBA, gl::UNSIGNED_BYTE,
            image.into_raw().as_ptr() as *const gl::types::GLvoid);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    };

    // Load the Image
    // TODO: Fix transparency values
    let image = match image::open("./src/frames/chest_empty_open_anim_f0.png") {
        Ok(image) => image.flipv().into_rgba(),
        Err(message) => panic!(format!("Image could not be loaded: {}", message))
    };
    
    // Create a texture
    let mut texture_id_2 = 0;
    unsafe { gl::GenTextures(1, &mut texture_id_2); };
    
    // Give the texture image data
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture_id_2);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGB as i32, 
            image.width() as i32, 
            image.height() as i32, 
            0, gl::RGBA, gl::UNSIGNED_BYTE,
            image.into_raw().as_ptr() as *const gl::types::GLvoid);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    };

    // Last Bit
    unsafe {

        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        // gl::Clear(gl::COLOR_BUFFER_BIT);
        // gl::DrawArrays(gl::TRIANGLES, 0, 6);
    };

    let player = world.create_entity()
        .with(Position{x: 0.0, y: 0.0, z: 0.0})
        .with(Velocity{x: 0.0, y: 0.0, z: 0.0})
        .with(Controlled)
        .with(Render{
            program_id: shader_program.id,
            texture_id: texture_id,
            vertex_buffer: square_vbo,
            vertices: square_vertices.clone()
        })
        .build();

    let item = world.create_entity()
        .with(Position{x: -0.5, y: 0.0, z: 0.0})
        .with(Render{
            program_id: shader_program.id,
            texture_id: texture_id_2,
            vertex_buffer: square_vbo,
            vertices: small_square_vertices.clone()
        })
        .build();

    // Enter the main event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {

        // Clear the event queue
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown {keycode: Some(key), ..} => {},
                Event::MouseButtonDown {mouse_btn: button, x, y, ..} => println!("Button Press: {}, {}, {:?}", x, y, button),
                _ => {}
            };
        }

        // Update Input State
        // TODO: Add this to a system?
        {
        let mut input_state = world.write_resource::<InputState>();
        input_state.up = event_pump.keyboard_state().is_scancode_pressed(Scancode::W);
        input_state.down = event_pump.keyboard_state().is_scancode_pressed(Scancode::S);
        input_state.left = event_pump.keyboard_state().is_scancode_pressed(Scancode::A);
        input_state.right = event_pump.keyboard_state().is_scancode_pressed(Scancode::D);
        input_state.action = event_pump.keyboard_state().is_scancode_pressed(Scancode::Space);
        }

        // Update Game States
        dispatcher.dispatch(&mut world);

        // Swap the buffers
        window.gl_swap_window();
    }
}
