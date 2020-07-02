#version 330 core

precision mediump float;

in vec2 UV;

uniform sampler2D textureSampler;

out vec4 Color;

void main() {

    Color = texture(textureSampler, UV);
}