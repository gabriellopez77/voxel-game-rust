#version 460 core

#include "../includes/globals.glsl"


layout(location = 0) in vec4 aVertex;
layout(location = 1) in ivec2 aPosition;
layout(location = 2) in ivec2 aSize;
layout(location = 3) in vec4 aInstanceTexCoords;
layout(location = 4) in uvec4 aColor;
layout(location = 5) in uint aTextureIdx;


layout(location = 0) out vec2 TexCoords;
layout(location = 1) out vec4 Color;
layout(location = 2) out uint TextureIdx;

void main() {
    gl_Position = globalUbo.uiProj * vec4(aVertex.xy * (aSize * globalUbo.pixelScale) + (aPosition * globalUbo.pixelScale), 0.f, 1.f);

    TexCoords = mix(aInstanceTexCoords.xy, aInstanceTexCoords.zw, aVertex.zw);
    Color = vec4(float(aColor.r), float(aColor.g), float(aColor.b), float(aColor.a)) / 255.0;
    TextureIdx = aTextureIdx;
}
