#version 100

uniform vec2 u_win_size;

attribute vec2 a_pos;
attribute vec4 a_color;

varying vec4 v_color;

void main() {
    vec2 xy = (a_pos / (u_win_size / 2.)) - 1.;
    xy.y *= -1.;

    gl_Position = vec4(xy, 0.0, 1.0);
    v_color = a_color;
}
