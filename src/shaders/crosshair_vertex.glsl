#version 140
in vec2 screen_position;

void main() {
    gl_Position = vec4(screen_position, 0, 1.0);
}