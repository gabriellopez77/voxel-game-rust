#version 460 core

layout (location = 0) in vec3 aVertex;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aPosition;
layout (location = 3) in float aCullface;

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

out vec3 Normal;

out float FogFactor;


void main()
{
	uint cullfaceValue = uint(aCullface);
	int index = gl_VertexID;

	if (index >= 12 && index <= 15 && !bool(cullfaceValue & (1 << 0))) return;
	else if (index >= 8 && index <= 11 && !bool(cullfaceValue & (1 << 1))) return;
	else if (index >= 16 && index <= 19 && !bool(cullfaceValue & (1 << 2))) return;
	else if (index >= 20 && index <= 23 && !bool(cullfaceValue & (1 << 3))) return;

	const vec3 size = vec3(12.f, 4.f, 12.f);
	//const vec3 positionOffset = vec3(0.f, -0.6f, 0.f);
	const vec3 pos = vec3(aPosition.x, 128.f, aPosition.y);

	vec4 viewSpace = camView * vec4((aVertex * size) + pos, 1.f);
    gl_Position = camProj * viewSpace;

    Normal = aNormal;

	FogFactor = calculateFog(viewSpace.xyz);
}