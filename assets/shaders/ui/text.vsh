#version 460 core

layout (location = 0) in vec4 aVertex;
layout (location = 1) in vec2 aPosition;
layout (location = 2) in vec2 aSize;
layout (location = 3) in vec4 aTexCoordsInstance;
layout (location = 4) in vec2 aAdvance;
layout (location = 5) in vec3 aColor;
layout (location = 6) in float aLayer;

layout(std140, binding = 0) uniform sla {
    mat4 projection;
    float pixelScale;
};

out vec2 TexCoords;
out vec3 Color;

void main()
{
    //const float pixelScale = 3.f;
    gl_Position = projection * vec4(((aPosition + aAdvance) * pixelScale) + aVertex.xy * (aSize * pixelScale), 0.f, 1.f);

    TexCoords = mix(aTexCoordsInstance.xy, aTexCoordsInstance.zw, aVertex.zw);
    Color = aColor / 255.f;
} 