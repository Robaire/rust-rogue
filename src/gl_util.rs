/// Generates a buffer on the GPU and returns its id
pub fn generate_buffer() -> u32 {
    let mut id = 0;

    unsafe {
        gl::GenBuffers(1, &mut id);
    };

    assert_ne!(id, 0);

    return id;
}

/// Sets the vertex data in a buffer
/// # Arguments
/// * `id` - Buffer ID
/// * `data` - Data to upload
pub fn set_buffer_data(id: u32, data: Vec<f32>) {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    };
}