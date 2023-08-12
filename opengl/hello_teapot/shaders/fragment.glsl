#version 150

in vec2 tex_coord;
in vec3 v_normal;
out vec4 color;

uniform sampler2D tex;
uniform vec3 u_light;

void main() {
    float brightness = max(dot(normalize(v_normal), normalize(u_light)), 0.005);

    vec3 regular_color = vec3(texture(tex, tex_coord));
    vec3 dark_color = 0.03 * regular_color;
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
