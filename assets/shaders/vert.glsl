#version 460 core

layout (location = 0) in vec4 aVertex;
layout (location = 1) in vec2 aPosition;
layout (location = 2) in vec2 aSize;
layout (location = 3) in vec4 aInstanceTexCoords;
layout (location = 4) in vec4 aColor;

layout(std140, binding = 0) uniform sla {
    mat4 projection;
};

out vec2 TexCoords;
out vec4 Color;

void main() {
    gl_Position = projection * vec4(aVertex.xy * aSize + aPosition, 0.f, 1.f);

    TexCoords = mix(aInstanceTexCoords.xy, aInstanceTexCoords.zw, aVertex.zw);
    Color = aColor / 255.f;
}

