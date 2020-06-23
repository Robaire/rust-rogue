extern crate gl;
extern crate sdl2;
use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate image;

pub mod shader;
use shader::{Shader, Program};

extern crate specs;
use specs::prelude::*;


struct Position { x: f64, y: f64, z: f64 }
impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct Velocity { x: f64, y: f64, z: f64 }
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct Orientation { x: f64, y: f64, z: f64, w: f64 }
impl Component for Orientation {
    type Storage = VecStorage<Self>;
}

struct Controlled;
impl Component for Controlled {
    type Storage = NullStorage<Self>;
}
impl Default for Controlled{
    fn default() -> Controlled {
       Controlled
    }
}

// struct ControlSystem;
// impl<'a> System<'a> for ControlSystem {
//     type SystemData = (WriteStorage<'a, Velocity>, ReadStorage<'a, Controlled>);

//     fn run(&mut self, (mut velocity): Self::SystemData) {
//         for (velocity) in (&mut velocity, )

//     }
// }

struct PhysicsSystem;
impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut position, velocity): Self::SystemData) {
        for (position, velocity) in (&mut position, &velocity).join() {
            
            position.x += velocity.x;
            position.y += velocity.y;
            position.z += velocity.z;
        }
    }
}

fn main() {

    // ECS Stuff
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    let player = world.create_entity()
        .with(Position{x: 0.0, y: 0.0, z: 0.0})
        .with(Velocity{x: 0.0, y: 0.0, z: 0.0})
        .build();

    let mut dispatcher = DispatcherBuilder::new().with(PhysicsSystem, "PhysicsSystem", &[]).build();
    dispatcher.dispatch(&mut world);
    
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
    gl_attributes.set_multisample_buffers(1);
    gl_attributes.set_multisample_samples(4);

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
    let mut vertices: Vec<f32> = vec![
        -0.3, -0.3, 0.0, 0.0, 0.0,
        0.3, 0.3, 0.0, 1.0, 1.0,
        -0.3, 0.3, 0.0, 0.0, 1.0,
        -0.3, -0.3, 0.0, 0.0, 0.0,
        0.3, -0.3, 0.0, 1.0, 0.0,
        0.3, 0.3, 0.0, 1.0, 1.0
    ];

    // Map Texture to Square
    let uv_coordinates: Vec<f32> = vec![
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        1.0, 0.0
    ];

    // Create a vertex buffer
    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo); };

    // Copy Vertex Data into the buffer
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    };

    // Create the vertex array and bind pointers to buffer information
    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); };
    unsafe { 
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

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
    let image = match image::open("./src/frames/big_zombie_idle_anim_f0.png") {
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
    
    let err = unsafe { gl::GetError() };
    println!("{}", err);

    println!("Last Bit");

    // Last Bit
    unsafe {

        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);

        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    };

    println!("Enter loop");

    // Enter the main event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {

        // Clear the event queue
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown {keycode: Some(key), ..} => {
                    println!("Key Press: {}", key);



                    let step = 0.03;

                    match key {
                        Keycode::W => {
                            vertices[1] += step;
                            vertices[6] += step;
                            vertices[11] += step;
                            vertices[16] += step;
                            vertices[21] += step;
                            vertices[26] += step;
                        },
                        Keycode::A => {
                            vertices[0] -= step;
                            vertices[5] -= step;
                            vertices[10] -= step;
                            vertices[15] -= step;
                            vertices[20] -= step;
                            vertices[25] -= step;
                        },
                        Keycode::S => {
                            vertices[1] -= step;
                            vertices[6] -= step;
                            vertices[11] -= step;
                            vertices[16] -= step;
                            vertices[21] -= step;
                            vertices[26] -= step;
                        },
                        Keycode::D => {
                            vertices[0] += step;
                            vertices[5] += step;
                            vertices[10] += step;
                            vertices[15] += step;
                            vertices[20] += step;
                            vertices[25] += step;
                        },
                        _ => {}
                    };

                    // Copy new vertex data into the buffer
                    unsafe {
                        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                        gl::BufferData(
                            gl::ARRAY_BUFFER,
                            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                            vertices.as_ptr() as *const gl::types::GLvoid,
                            gl::STATIC_DRAW
                        );
                        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    };
                    
                },
                Event::MouseButtonDown {mouse_btn: button, x, y, ..} => println!("Button Press: {}, {}, {:?}", x, y, button),
                _ => {}
            };
        }

        // Update Game State
        dispatcher.dispatch(&mut world);
        
        // Draw things
        unsafe { 
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 6); 
        };

        // Swap the buffers
        window.gl_swap_window();
    }
}
