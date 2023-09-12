#version 150

in vec3 position;
in vec3 normal;

out vec2 tex_coord;
out vec3 v_normal;

uniform mat4 matrix;
uniform mat4 perspective;

void main() {
    tex_coord = vec2((200.0 - (100.0 + position.x)) / 200.0, (100.0 - (position.y + 50.0)) / 100.0);
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = perspective * matrix * vec4(position, 1.0);
}
