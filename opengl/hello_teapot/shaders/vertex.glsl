#version 140

in vec3 position;
in vec3 normal;

out vec2 tex_coord;
uniform mat4 matrix;

void main() {
    tex_coord = vec2((200.0 - (100.0 + position.x)) / 200.0, (100.0 - (position.y + 50.0)) / 100.0);
    gl_Position = matrix * vec4(position, 1.0);
}
