#version 460 core

layout (location = 0) in vec2 aVertex;
layout (location = 1) in vec3 aColor;

out vec3 Color;

void main() {
    gl_Position = vec4(aVertex, 0.f, 1.f);
    
    Color = aColor;
}

