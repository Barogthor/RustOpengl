#version  330 core

in vec3 position;
in vec3 normal;

out vec3 oNormal;
out vec3 fragPos;

uniform mat4 vp;
uniform mat4 model;

void main() {
    gl_Position = vp * model * vec4(position, 1.0);
    fragPos = vec3(model * vec4(position, 1.0));
    oNormal = mat3(transpose(inverse(model))) * normal;// normal matrix costly, should calculate on cpu and use uniform
}
