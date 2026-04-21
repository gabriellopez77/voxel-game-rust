#version 460 core

layout (location = 0) in vec2 aVertex;
layout (location = 1) in vec3 aColor;

uniform mat4 projection;
uniform mat4 view;

out vec3 Color;

void main() {
    gl_Position = projection * view * vec4(aVertex * vec2(10, 10), 0.f, 1.f);
    
    Color = aColor;
}

