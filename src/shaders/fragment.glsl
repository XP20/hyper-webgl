#version 300 es

precision highp float;

in vec2 f_texcoord;
in vec3 f_normal;

out vec4 outColor;

void main() {
    // outColor = vec4(1.0, 1.0, 1.0, 1.0);
    outColor = vec4(f_normal, 1.0);
}
