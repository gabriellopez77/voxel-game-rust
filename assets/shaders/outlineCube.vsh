#version 460 core

#include "includes/globals.glsl"


layout(location = 0) in ivec3 aVertex;
layout(location = 1) in vec3 aPosition;
layout(location = 2) in vec3 aSize;
layout(location = 3) in uvec4 aColor;

layout(location = 0) out vec4 Color;

void main() {
    gl_Position = globalUbo.camViewProj * vec4(aVertex * aSize + aPosition, 1.f);

    Color = aColor / 255.0;
}
