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

    let mut vertices: Vec<f32> = vec![
        -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0,
        0.0,
    ];

    for vertex in &mut vertices {
        *vertex *= 0.3;
    }

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
        gl::TexParameteri(
            gl::TEXTURE_2D_ARRAY,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as i32
        );
        gl::TexParameteri(
            gl::TEXTURE_2D_ARRAY,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR as i32
        );
    }

    // Create an entity
    world
        .create_entity()
        .with(Position::new())
        .with(Size::default())
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

    create_entity(&mut world, shader_program.id);

    // // Create Square
    // let square_vertices: Vec<f32> = vec![
    //     -0.3, -0.3, 0.0, 0.0, 0.0, 0.3, 0.3, 0.0, 1.0, 1.0, -0.3, 0.3, 0.0, 0.0, 1.0, -0.3, -0.3,
    //     0.0, 0.0, 0.0, 0.3, -0.3, 0.0, 1.0, 0.0, 0.3, 0.3, 0.0, 1.0, 1.0,
    // ];

    // // Create Square
    // let small_square_vertices: Vec<f32> = vec![
    //     -0.1, -0.1, 0.0, 0.0, 0.0, 0.1, 0.1, 0.0, 1.0, 1.0, -0.1, 0.1, 0.0, 0.0, 1.0, -0.1, -0.1,
    //     0.0, 0.0, 0.0, 0.1, -0.1, 0.0, 1.0, 0.0, 0.1, 0.1, 0.0, 1.0, 1.0,
    // ];

    // // Create a vertex buffer
    // let mut square_vbo = 0;
    // unsafe {
    //     gl::GenBuffers(1, &mut square_vbo);
    // };

    // // Copy Vertex Data into the buffer
    // unsafe {
    //     gl::BindBuffer(gl::ARRAY_BUFFER, square_vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (square_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
    //         square_vertices.as_ptr() as *const gl::types::GLvoid,
    //         gl::STATIC_DRAW
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    // };

    // // Create the vertex array and bind pointers to buffer information
    // let mut vao = 0;
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    // };
    // unsafe {
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, square_vbo);

    //     gl::EnableVertexAttribArray(0);
    //     gl::EnableVertexAttribArray(1);

    //     gl::VertexAttribPointer(
    //         0,
    //         3,
    //         gl::FLOAT,
    //         gl::FALSE,
    //         (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
    //         std::ptr::null()
    //     );

    //     let offset = 3;
    //     gl::VertexAttribPointer(
    //         1,
    //         2,
    //         gl::FLOAT,
    //         gl::FALSE,
    //         (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
    //         (offset * std::mem::size_of::<f32>()) as *const std::ffi::c_void
    //     );
    // };

    // // Load the animation as a texture array
    // let idle_animation = match image::open("./src/animations/ogre_idle_animation.png") {
    //     Ok(image) => image.flipv().into_rgba(),
    //     Err(message) => panic!(format!("Image could not be loaded: {}", message))
    // };

    // // Create a texture array and store image data in it
    // let mut texture_array_id = 0;

    // unsafe {
    //     gl::GenTextures(1, &mut texture_array_id);
    //     gl::BindTexture(gl::TEXTURE_2D_ARRAY, texture_array_id);
    //     gl::TexStorage3D(gl::TEXTURE_2D_ARRAY, 1, gl::RGBA8, 22, 28, 4);

    //     gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 22 * 4);
    //     gl::PixelStorei(gl::UNPACK_IMAGE_HEIGHT, 28);

    //     gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 0);
    //     gl::TexSubImage3D(
    //         gl::TEXTURE_2D_ARRAY,
    //         0,
    //         0,
    //         0,
    //         0,
    //         22,
    //         28,
    //         1,
    //         gl::RGBA,
    //         gl::UNSIGNED_BYTE,
    //         idle_animation.as_ptr() as *const gl::types::GLvoid
    //     );

    //     gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 22);
    //     gl::TexSubImage3D(
    //         gl::TEXTURE_2D_ARRAY,
    //         0,
    //         0,
    //         0,
    //         1,
    //         22,
    //         28,
    //         1,
    //         gl::RGBA,
    //         gl::UNSIGNED_BYTE,
    //         idle_animation.as_ptr() as *const gl::types::GLvoid
    //     );

    //     gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 44);
    //     gl::TexSubImage3D(
    //         gl::TEXTURE_2D_ARRAY,
    //         0,
    //         0,
    //         0,
    //         2,
    //         22,
    //         28,
    //         1,
    //         gl::RGBA,
    //         gl::UNSIGNED_BYTE,
    //         idle_animation.as_ptr() as *const gl::types::GLvoid
    //     );

    //     gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 66);
    //     gl::TexSubImage3D(
    //         gl::TEXTURE_2D_ARRAY,
    //         0,
    //         0,
    //         0,
    //         3,
    //         22,
    //         28,
    //         1,
    //         gl::RGBA,
    //         gl::UNSIGNED_BYTE,
    //         idle_animation.as_ptr() as *const gl::types::GLvoid
    //     );

    //     gl::TexParameteri(
    //         gl::TEXTURE_2D_ARRAY,
    //         gl::TEXTURE_MAG_FILTER,
    //         gl::NEAREST as i32
    //     );
    //     gl::TexParameteri(
    //         gl::TEXTURE_2D_ARRAY,
    //         gl::TEXTURE_MIN_FILTER,
    //         gl::LINEAR as i32
    //     );
    // };

    // // Load the Image
    // let image = match image::open("./src/frames/chest_empty_open_anim_f0.png") {
    //     Ok(image) => image.flipv().into_rgba(),
    //     Err(message) => panic!(format!("Image could not be loaded: {}", message))
    // };

    // // Create a texture
    // let mut chest_texture_id = 0;
    // unsafe {
    //     gl::GenTextures(1, &mut chest_texture_id);
    // };

    // // Give the texture image data
    // unsafe {
    //     gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
    //     gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 0);
    //     gl::BindTexture(gl::TEXTURE_2D, chest_texture_id);
    //     gl::TexImage2D(
    //         gl::TEXTURE_2D,
    //         0,
    //         gl::RGBA8 as i32,
    //         image.width() as i32,
    //         image.height() as i32,
    //         0,
    //         gl::RGBA,
    //         gl::UNSIGNED_BYTE,
    //         image.into_raw().as_ptr() as *const gl::types::GLvoid
    //     );
    //     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    //     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    // };

    // Last Bit
    unsafe {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        gl::Enable(gl::CULL_FACE);

        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    };

    /*
    // Create entities in the world
    let player = world
        .create_entity()
        .with(Position::new())
        .with(Velocity::new())
        .with(Controlled)
        .with(Animation::new(4))
        .with(Draw::new(
            shader_program.id,
            square_vbo,
            square_vertices.clone(),
            DrawType::Dynamic {
                texture_id: texture_array_id,
                layer: 0
            }
        ))
        .build();

    let chest = world
        .create_entity()
        .with(Position::new())
        .with(Draw::new(
            static_shader_program.id,
            square_vbo,
            small_square_vertices.clone(),
            DrawType::Static {
                texture_id: chest_texture_id
            }
        ))
        .build();
    */

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
