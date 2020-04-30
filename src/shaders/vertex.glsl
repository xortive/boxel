#version 140
uniform mat4 persp_matrix;
uniform mat4 view_matrix;
in vec3 position;
in vec3 color; //from instance VBO
in vec3 world_position; //From instance VBO
in vec3 normal;
out vec3 v_position;
out vec3 v_normal;
out vec3 v_color;
void main() {
    v_position = position + world_position;
    v_normal = normal;
    v_color = color;
    gl_Position = persp_matrix * view_matrix * vec4(v_position, 1.0);
}