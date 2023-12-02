#version 300 es

// attribute vec3 coordinates;

in vec4 position;

// uniform mat4 transform;

void main() {
    // gl_Position = transform * vec4(coordinates, 1.0);
    // gl_Position = vec4(coordinates, 1.0);
    gl_Position = position;
}
