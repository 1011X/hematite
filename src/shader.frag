#version 120
uniform sampler2D s_texture;

varying vec2 v_tex_coord;
varying vec3 v_color;

void main() {
    vec4 tex_color = texture2D(s_texture, v_tex_coord);
    if(tex_color.a == 0.0) // Discard transparent pixels.
        discard;
    gl_FragColor = tex_color * vec4(v_color, 1.0);
}
