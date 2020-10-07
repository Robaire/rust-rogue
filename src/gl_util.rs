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
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    };
}

/// Bind a buffer
/// # Arguments
/// * `id` - Buffer ID
pub fn bind_buffer(id: u32) {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, id);
    }
}

/// Generates a vertex attribute array on the GPU
pub fn generate_vertex_array() -> u32 {
    let mut id = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut id);
    };

    assert_ne!(id, 0);

    return id;
}

/// Bind an attribute array
/// # Arguments
/// * `id` - Vertex Array ID
pub fn bind_array(id: u32) {
    unsafe {
        gl::BindVertexArray(id);
    }
}

/// Set vertex attribute array
/// # Arguments
/// * `buffer` - Buffer vertex data is stored in
/// * `id` - Vertex Array ID
/// * `index` - Vertex Array Index to modify
/// * `size` - The number of components per vertex
pub fn set_vertex_array_pointer(buffer: u32, id: u32, index: u32, size: i32) {
    if size > 4 {
        panic!("Size must be 1, 2, 3, or 4");
    }

    unsafe {
        gl::BindVertexArray(id);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);

        gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(index);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}
