#version 100

precision mediump float;

uniform vec4 u_color;
uniform float u_dist_factor;
uniform sampler2D u_texture;

varying vec2 v_uv;

float median(vec3 col) {
    return max(min(col.r, col.g), min(max(col.r, col.g), col.b));
}

void main() {
    // TODO: seems like it's BGRA instead of RGBA
    float distance = u_dist_factor * (median(texture2D(u_texture, v_uv).rgb) - 0.5);
    float opacity = 1. - clamp(distance + 0.5, 0.0, 1.0);

    gl_FragColor = vec4(u_color.rgb, u_color.a * opacity);
}
