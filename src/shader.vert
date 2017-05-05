#version 120
uniform mat4 u_projection, u_view;

attribute vec2 at_tex_coord;
attribute vec3 at_color, at_position;

varying vec2 v_tex_coord;
varying vec3 v_color;

void main() {
    v_tex_coord = at_tex_coord;
    v_color = at_color;
    gl_Position = u_projection * u_view * vec4(at_position, 1.0);
}
