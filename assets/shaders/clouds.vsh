#version 460 core

#include "includes/globals.glsl"
#include "includes/utils.glsl"


layout(location = 0) in ivec3 aVertex;
layout(location = 1) in ivec3 aNormal;
layout(location = 2) in int aFaceId;
layout(location = 3) in vec2 aPosition;
layout(location = 4) in uint aCullfaces;


layout(location = 0) out vec3 Normal;
layout(location = 1) out float FogFactor;

void main()
{
	const uint FACES_MASK[] = uint[](1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5);

	if ((aCullfaces & FACES_MASK[aFaceId]) == 0) {
	    gl_Position = vec4(0.0 / 0.0);
		return;
	}

	const vec3 size = vec3(12.f, 4.f, 12.f);
	const vec3 pos = vec3(aPosition.x, 128.f, aPosition.y);

	vec4 viewSpace = globalUbo.camView * vec4((aVertex * size) + pos, 1.f);
    gl_Position = globalUbo.camProj * viewSpace;

    Normal = aNormal;
	FogFactor = calculateFog(viewSpace.xyz);
}
