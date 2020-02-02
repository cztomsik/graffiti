#version 100

uniform mat3 u_projection;
uniform mat3 u_transform;

attribute vec2 a_pos;
attribute vec4 a_color;

varying vec4 v_color;

void main() {
    vec3 pos = vec3(a_pos, 1.0);
    gl_Position = vec4(pos * u_transform * u_projection, 1.0);
    v_color = a_color;
}
