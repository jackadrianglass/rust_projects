#version 140

in vec3 f_colour;
out vec4 color;

void main() {
    color = vec4(f_colour, 1.0);
}
