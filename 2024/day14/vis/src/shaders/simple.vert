#version 460 core

layout (location = 0) in vec4 vertex;

layout (location = 0) uniform mat4 model;
layout (location = 1) uniform mat4 projection;

void main() {
    gl_Position = projection * model * vec4(vertex.xy, 0.0, 1.0);
}
