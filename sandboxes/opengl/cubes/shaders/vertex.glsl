#version 140

in vec3 position;
in vec3 colour;

out vec3 f_colour;

uniform mat4 matrix;

void main() {
    f_colour = colour;
    gl_Position = matrix * vec4(position, 1.0);
}
