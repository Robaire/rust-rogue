extern crate gl;
extern crate sdl2;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use sdl2::video::GLProfile;

extern crate image;

pub mod shader;
use shader::{Program, Shader};

pub mod component_system;
pub mod gl_util;

extern crate specs;
use specs::prelude::*;
use specs::WorldExt;

use std::ffi::CString;

extern crate nalgebra;
use nalgebra::Orthographic3;

fn init_sdl() -> (sdl2::Sdl, sdl2::video::Window, sdl2::video::GLContext) {
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

    // Create the window
    let window = match video_subsystem
        .window("Rust Rouge", 600, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(message) => panic!(format!("Failed to create window: {}", message))
    };

    // Create the OpenGL Context
    let gl_context = match window.gl_create_context() {
        Ok(context) => context,
        Err(message) => panic!(format!("Failed to create OpenGL Context: {}", message))
    };

    // Load the OpenGL Functions
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    (sdl_context, window, gl_context)
}

fn setup_ecs<'a>() -> (specs::World, specs::Dispatcher<'a, 'a>) {
    use component_system::*;

    // Create the world
    let mut world = World::new();

    // Register Components
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Size>();
    world.register::<components::Controlled>();
    world.register::<components::Animate>();
    world.register::<components::Drawn>();

    // Insert Resources
    world.insert(resources::DeltaTime::default());
    world.insert(resources::InputState::new());

    // Create the dispatcher
    let dispatcher = DispatcherBuilder::new()
        // Add parallel systems
        .with(systems::TimeSystem, "TimeSystem", &[])
        .with(systems::ControlSystem, "ControlSystem", &["TimeSystem"])
        .with(systems::PhysicsSystem, "PhysicsSystem", &["ControlSystem"])
        .with(systems::AnimateSystem, "AnimationSystem", &["TimeSystem"])
        // Add serial systems
        .with_thread_local(systems::DrawSystem)
        .build();

    (world, dispatcher)
}

fn create_shader_program() -> Program {
    // Load shaders
    let vertex_shader = match Shader::new_from_file("./src/shaders/entity.vert", gl::VERTEX_SHADER)
    {
        Ok(shader) => shader,
        Err(message) => panic!(format!("Failed to create vertex shader: {}", message))
    };

    let fragment_shader =
        match Shader::new_from_file("./src/shaders/entity.frag", gl::FRAGMENT_SHADER) {
            Ok(shader) => shader,
            Err(message) => panic!(format!("Failed to create fragment shader: {}", message))
        };

    // Create shader program
    let shader_program = match Program::new()
        .attach_shader(&vertex_shader)
        .attach_shader(&fragment_shader)
        .link()
    {
        Ok(program) => program,
        Err(message) => panic!(format!("Failed to create shader program: {}", message))
    };

    // Use shader program
    shader_program.set_used();

    return shader_program;
}

fn create_entity(world: &mut specs::World, program: u32) {
    use component_system::components::*;

    let vertices: Vec<f32> = vec![
        -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0,
        0.0,
    ];

    let texture_vertices: Vec<f32> =
        vec![0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0];

    let texture = match image::open("./src/frames/skull.png") {
        Ok(image) => image.flipv().into_rgba(),
        Err(message) => panic!(format!("Image could not be loaded: {}", message))
    };

    let texture_id = gl_util::generate_texture();
    gl_util::bind_texture(texture_id);

    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            16,
            16,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            texture.as_ptr() as *const gl::types::GLvoid
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    }

    // Create an entity
    world
        .create_entity()
        .with(Position::new_xyz(0.0, 0.0, 0.0))
        .with(Velocity::new())
        .with(Controlled)
        .with(Size::new(0.3, 0.3))
        .with(Drawn::new(program, texture_id, vertices, texture_vertices))
        .build();
}

fn main() {
    // Initialize SDL and create a window
    let (sdl_context, window, _gl_context) = init_sdl();

    // Setup the ECS
    let (mut world, mut dispatcher) = setup_ecs();

    // Create the shader program
    let shader_program = create_shader_program();

    // Add entities to the world
    create_entity(&mut world, shader_program.id);

    // Set the projection matrix
    let projection_id = unsafe {
        gl::GetUniformLocation(
            shader_program.id,
            CString::new("projection").unwrap().as_ptr()
        )
    };

    let aspect = 1.0;
    let projection = Orthographic3::new(-aspect, aspect, -1.0, 1.0, -1.0, 1.0);

    // Write the projection to the gpu
    unsafe {
        gl::UniformMatrix4fv(
            projection_id,
            1,
            gl::FALSE,
            projection.to_homogeneous().as_slice().as_ptr()
        );
    };

    // Last Bit
    unsafe {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        gl::Enable(gl::CULL_FACE);

        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    };

    // Enter the main event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        // Clear the event queue
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {},
                Event::MouseButtonDown {
                    mouse_btn: button,
                    x,
                    y,
                    ..
                } => println!("Button Press: {}, {}, {:?}", x, y, button),
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(x, y) => unsafe {
                        gl::Viewport(0, 0, x, y);

                        // Compute the projection
                        let aspect = x as f32 / y as f32;
                        let projection = Orthographic3::new(-aspect, aspect, -1.0, 1.0, -1.0, 1.0);

                        // Write the projection to the gpu
                        shader_program.set_used();
                        gl::UniformMatrix4fv(
                            projection_id,
                            1,
                            gl::FALSE,
                            projection.to_homogeneous().as_slice().as_ptr()
                        );
                    },
                    _ => {}
                },
                Event::Quit { .. } => break 'main_loop,
                _ => {}
            };
        }

        // Update Input State
        {
            use component_system::resources::InputState;
            let mut input_state = world.write_resource::<InputState>();
            input_state.up = event_pump.keyboard_state().is_scancode_pressed(Scancode::W);
            input_state.down = event_pump.keyboard_state().is_scancode_pressed(Scancode::S);
            input_state.left = event_pump.keyboard_state().is_scancode_pressed(Scancode::A);
            input_state.right = event_pump.keyboard_state().is_scancode_pressed(Scancode::D);
            input_state.action = event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::Space);
        }

        // Update Game States
        dispatcher.dispatch(&mut world);

        // Swap the buffers
        window.gl_swap_window();
    }
}
