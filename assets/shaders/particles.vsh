#version 460 core

#include "includes/globals.glsl"
#include "includes/utils.glsl"


layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec2 aTexCoords;
layout(location = 2) in vec3 aPosition;
layout(location = 3) in vec3 aScale;
layout(location = 4) in vec3 aRotation;
layout(location = 5) in vec4 aInstanceTexCoords;
layout(location = 6) in uint aTextureIdx;


layout(location = 0) out vec2 TexCoords;
layout(location = 1) out uint TextureIdx;
layout(location = 2) out float FogFactor;

void main()
{
    mat4 matrix = buildTransform(aPosition, aScale, aRotation);

    vec4 viewSpace = globalUbo.camView * matrix * vec4(aVertex, 1.f);
    gl_Position = globalUbo.camProj * viewSpace;

    TexCoords = mix(aInstanceTexCoords.xy, aInstanceTexCoords.zw, aTexCoords);
    TextureIdx = aTextureIdx;
    FogFactor = calculateFog(viewSpace.xyz);
}
