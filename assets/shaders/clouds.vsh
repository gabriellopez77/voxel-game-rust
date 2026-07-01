#version 460 core

#include "includes/globals.glsl"
#include "includes/utils.glsl"


layout(location = 0) in ivec3 aVertex;
layout(location = 1) in ivec3 aNormal;
layout(location = 2) in vec2 aPosition;
layout(location = 3) in uint aCullface;


layout(location = 0) out vec3 Normal;

layout(location = 1) out float FogFactor;

void main()
{
	uint cullfaceValue = uint(aCullface);
	int index = gl_VertexIndex;

	if (index >= 12 && index <= 15 && !bool(cullfaceValue & (1 << 0))) return;
	else if (index >= 8 && index <= 11 && !bool(cullfaceValue & (1 << 1))) return;
	else if (index >= 16 && index <= 19 && !bool(cullfaceValue & (1 << 2))) return;
	else if (index >= 20 && index <= 23 && !bool(cullfaceValue & (1 << 3))) return;

	const vec3 size = vec3(12.f, 4.f, 12.f);
	const vec3 pos = vec3(aPosition.x, 128.f, aPosition.y);

	vec4 viewSpace = globalUbo.camView * vec4((aVertex * size) + pos, 1.f);
    gl_Position = globalUbo.camProj * viewSpace;

    Normal = vec3(float(aNormal.x), float(aNormal.y), float(aNormal.z));

	FogFactor = calculateFog(viewSpace.xyz);
}
