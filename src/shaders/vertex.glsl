#version 300 es

in vec3 position;
in vec2 texcoord;
in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec2 f_texcoord;
out vec3 f_normal;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    f_texcoord = texcoord;
    f_normal = normal;
}
