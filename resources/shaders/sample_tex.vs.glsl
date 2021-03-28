#version  330 core

in vec3 position;
in vec2 tex_coords;

out vec2 TexCoords;

void main() {
    gl_Position = vec4(position.x, position.y, position.z, 1.0);

    TexCoords = tex_coords;
}
