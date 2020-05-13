#version 140
uniform mat4 persp_matrix;
uniform mat4 view_matrix;
in vec4 color; //from instance VBO
in vec3 world_position; //From instance VBO
out vec3 v_position;
out vec3 v_normal;
out vec4 v_color;
void main() {
    v_position = world_position;
    v_normal = vec3(0);
    v_color = color;
    gl_Position = persp_matrix * view_matrix * vec4(v_position, 1.0);
}