#version 460 core

#extension GL_ARB_shader_draw_parameters : enable

#include "includes/globals.glsl"
#include "includes/utils.glsl"


layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoords;
layout(location = 3) in uint aFlags;
layout(location = 4) in uint aLightLevels;


layout(location = 0) out vec3 Normal;
layout(location = 1) out vec2 TexCoords;
layout(location = 2) out float Shade;
layout(location = 3) out float AoLevel;
layout(location = 4) out vec2 LightLevels;
layout(location = 5) out float FogFactor;
layout(location = 6) out flat uint DrawId;

void main() {
    vec4 viewSpace = globalUbo.camView * vec4(aVertex, 1.f);;
    gl_Position = globalUbo.camProj * viewSpace;

    Normal = aNormal;
    TexCoords = aTexCoords;
    Shade = extractShadingValue(aFlags) ? calculateShading(aNormal) : 1.0;
    AoLevel = extractAoLevels(aFlags);
    LightLevels = extractLightLevels(aLightLevels);
    FogFactor = calculateFog(viewSpace.xyz);
    DrawId = uint(gl_DrawID);
}
