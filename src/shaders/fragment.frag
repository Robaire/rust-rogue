#version 330 core

precision mediump float;

in vec2 texture_coordinate;

uniform sampler2DArray texture_sampler;
uniform int layer;

out vec4 Color;

void main() {

    Color = texture(texture_sampler, vec3(texture_coordinate, layer));
}