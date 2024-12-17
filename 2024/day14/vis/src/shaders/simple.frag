#version 460 core

out vec4 color;

layout (location = 2) uniform vec3 in_color;

void main() {
    color = vec4(in_color, 1.0);
}
