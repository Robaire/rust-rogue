#version 330 core

precision mediump float;

in vec2 UV;

uniform sampler2D textureSampler;

out vec4 Color;

void main() {

    vec3 sample = texture(textureSampler, UV).rgb;
    float alpha = 1.0f;
    if(sample.r == 0.0f && sample.g == 0.0f && sample.b == 0.0f) {
        alpha = 0.0f;
    }
    Color = vec4(sample, alpha);
}