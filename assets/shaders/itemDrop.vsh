#version 460 core

#include "includes/globals.glsl"
#include "includes/utils.glsl"


layout(push_constant) uniform PushConstants {
    mat4 model;
    uint lightLevels;
} push;

layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoords;

layout(location = 0) out vec3 Normal;
layout(location = 1) out vec2 TexCoords;
layout(location = 2) out vec2 LightLevels;
layout(location = 3) out float FogFactor;

void main()
{
    vec4 viewSpace = globalUbo.camView * push.model * vec4(aVertex, 1.0);
    gl_Position = globalUbo.camProj * viewSpace;

    Normal = aNormal;
    TexCoords = aTexCoords;
    LightLevels = extractLightLevels(push.lightLevels);
    FogFactor = calculateFog(viewSpace.xyz);
}
