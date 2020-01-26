#version 100

uniform mat3 u_projection;
uniform vec2 u_pos;

attribute vec2 a_pos;
attribute vec2 a_uv;

varying vec2 v_uv;

void main() {
    vec3 pos = vec3(a_pos + u_pos, 1.0);
    gl_Position = vec4(pos * u_projection, 1.0);
    v_uv = a_uv;
}
