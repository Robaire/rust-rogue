#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 vertexUV;

uniform mat4 projection;

out vec2 UV;

void main() {

    gl_Position = projection * vec4(Position, 1.0);

    UV = vertexUV;
}