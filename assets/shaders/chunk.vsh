#version 460 core

#define AO_LEVEL_FLAG (0x3)
#define SHADE_FLAG (0x4)

layout (location = 0) in vec3 aVertex;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in float aFlags;

layout(std140, binding = 0) uniform shaderData {
    mat4 uiProj;
    float pixelScale;
    mat4 camProj;
	mat4 camView;
	mat4 camViewProj;
	mat4 viewNoTranslation;
};

layout (std140, binding = 1) uniform worldData {
    // fog
    vec3 skyColor;
    vec3 fogColor;
    vec3 lightColor;
    vec3 darknessColor;
    vec3 ambientColor;
    vec3 cloudsColor;
    float fogDistance;
    float fogDensity;
    int fogEnable;
};

float calculateFog(vec3 viewSpace)
{
    float distanceFromCamera = length(viewSpace);
    return exp(-pow(distanceFromCamera * fogDistance, fogDensity));
}

uniform vec3 pos;

out vec3 Normal;
out vec2 TexCoords;

flat out int Shade;
out float AoLevel;

out float FogFactor;

void main() {
    vec4 viewSpace = camView * vec4(aVertex + pos, 1.f);;
    gl_Position = camProj * viewSpace;

   	// flags and Ao level
	int flagsValue = int(aFlags);
    Shade = (flagsValue & SHADE_FLAG) >> 2;
    AoLevel = ((flagsValue & AO_LEVEL_FLAG) >> 0) / 3.f;

    Normal = aNormal;
    TexCoords = aTexCoords;
    FogFactor = calculateFog(viewSpace.xyz);
}
