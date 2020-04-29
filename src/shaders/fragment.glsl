#version 140
in vec3 v_normal;
in vec3 v_color;
out vec4 f_color;
const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
void main() {
  float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
  vec3 color = (0.3 + 0.7 * lum) * v_color;
  f_color = vec4(color, 1.0);
}