#version 300 es

precision highp float;

in vec3 position;
in vec2 texcoord;
in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform highp int toggle;

out vec2 f_texcoord;
out vec3 f_normal;

float sigmoid(float x, float k) {
    return x / abs(x) * (1.0 - exp(-1.0 * k * abs(x)));
}

vec3 sigmoid(vec3 x, float k) {
    return vec3(
        sigmoid(x.x, k),
        sigmoid(x.y, k),
        sigmoid(x.z, k)
    );
}

void main() {
    vec4 pre_pos = projection * view * model * vec4(position, 1.0);
    if (toggle == 1) {
        gl_Position = vec4(sigmoid(pre_pos.xyz, 1.0), 1.0);
    } else {
        gl_Position = pre_pos;
    }
    f_texcoord = texcoord;
    f_normal = normal;
}
