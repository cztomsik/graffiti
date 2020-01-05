#version 100

precision mediump float;

varying vec4 v_color;

void main() {
    // TODO: move division to VS
    gl_FragColor = v_color / 256.;
}
