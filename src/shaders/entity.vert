#version 330 core

layout (location = 0) in vec3 entity_vertex;
layout (location = 1) in vec2 texture_vertex;

uniform mat4 projection;
uniform vec3 position;
uniform vec2 size;

out vec2 texture_coordinate;

void main() {

    texture_coordinate = texture_vertex;

    // Scale the entity vertices by the entity size
    mat3 scale = mat3(
        size.x / 2.0, 0.0, 0.0,
        0.0, size.y / 2.0, 0.0,
        0.0, 0.0, 1.0);

    gl_Position = projection * vec4((scale * entity_vertex) + position, 1.0);
}