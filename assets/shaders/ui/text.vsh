#version 460 core

layout (location = 0) in vec4 aVertex;
layout (location = 1) in vec2 aPosition;
layout (location = 2) in vec2 aSize;
layout (location = 3) in vec4 aTexCoordsInstance;
layout (location = 4) in vec2 aAdvance;
layout (location = 5) in vec3 aColor;
layout (location = 6) in float aLayer;

layout (std140, binding = 1) uniform shader_matrix {
    mat4 projection;
    int pixelScale;
};

out vec2 TexCoords;
out vec3 Color;

void main()
{
    gl_Position = projection * vec4(((aPosition + aAdvance) * pixelScale) + aVertex.xy * (aSize * pixelScale), aLayer - 1000, 1.f);

    TexCoords = mix(aTexCoordsInstance.xy, aTexCoordsInstance.zw, aVertex.zw) / 128.f;
    Color = aColor / 255.f;
} 