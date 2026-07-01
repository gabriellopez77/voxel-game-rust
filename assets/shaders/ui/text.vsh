#version 460 core

#include "../includes/globals.glsl"


layout(location = 0) in vec4 aVertex;
layout(location = 1) in ivec2 aPosition;
layout(location = 2) in uvec2 aSize;
layout(location = 3) in vec4 aTexCoordsInstance;
layout(location = 4) in ivec2 aAdvance;
layout(location = 5) in uvec3 aColor;


layout(location = 0) out vec2 TexCoords;
layout(location = 1) out vec3 Color;

void main()
{
    gl_Position = globalUbo.uiProj * vec4(((aPosition + aAdvance) * globalUbo.pixelScale) + aVertex.xy * (aSize * globalUbo.pixelScale), 0.f, 1.f);

    TexCoords = mix(aTexCoordsInstance.xy, aTexCoordsInstance.zw, aVertex.zw);
    Color = vec3(float(aColor.r), float(aColor.g), float(aColor.b)) / 255.f;
}
