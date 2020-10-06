#version 330 core

precision mediump float;

in vec2 texture_coordinate;

uniform sampler2D texture_sampler;

out vec4 Color;

void main() {
    Color = texture(texture_sampler, texture_coordinate);
}