#version 300 es

precision highp float;

in vec2 f_texcoord;
in vec3 f_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float time;
uniform highp int toggle;
uniform highp int rgb;
uniform highp float hue;
uniform highp float ambient;
uniform highp float sat;

out vec4 outColor;

vec3 rgb2hsv(vec3 c) {
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    // outColor = vec4(f_normal, 1.0);
    float hue2 = mod((time / 3000.0), 1.0);
    vec3 color = hsv2rgb(vec3(hue2, sat, 1.0));
    if (rgb == 1) {
        if (hue != 0.0) {
            color = hsv2rgb(vec3(hue, sat, 1.0));
        } else {
            color = vec3(1.0, 1.0, 1.0);
        }
    }
    float light = ambient + abs(dot(normalize((view * vec4(0.0, 0.0, 1.0, 1.0)).xyz), normalize((model * vec4(f_normal, 0.0)).xyz)));
    outColor = vec4(color * light, 1.0);
}
