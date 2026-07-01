#version 460 core

#include "includes/globals.glsl"
#include "includes/utils.glsl"

#define AO_LEVEL_FLAG (0x3)
#define SHADE_FLAG (0x4)

layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoords;
layout(location = 3) in uint aFlags;


layout(location = 0) out vec3 Normal;
layout(location = 1) out vec2 TexCoords;

layout(location = 2) flat out int Shade;
layout(location = 3) out float AoLevel;

layout(location = 4) out float FogFactor;

void main() {
    vec4 viewSpace = globalUbo.camView * vec4(aVertex, 1.f);;
    gl_Position = globalUbo.camProj * viewSpace;

   	// flags and Ao level
	int flagsValue = int(aFlags);
    Shade = (flagsValue & SHADE_FLAG) >> 2;
    AoLevel = ((flagsValue & AO_LEVEL_FLAG) >> 0) / 3.f;

    Normal = aNormal;
    TexCoords = aTexCoords;
    FogFactor = calculateFog(viewSpace.xyz);
}