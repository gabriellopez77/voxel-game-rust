#version 460 core

layout (location = 0) in vec3 aVertex;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aPosition;


layout(std140, binding = 0) uniform shaderData {
	mat4 uiProj;
	float pixelScale;
	mat4 camProj;
	mat4 camView;
	mat4 camViewProj;
	mat4 viewNoTranslation;
};

const vec3 teste[24] = vec3[24]
(
	// up
	vec3(1, 1, 0),
	vec3(0, 1, 0),
	vec3(0, 1, 1),
	vec3(1, 1, 1),

	// down
	vec3(1, 0, 1),
	vec3(0, 0, 1),
	vec3(0, 0, 0),
	vec3(1, 0, 0),

	// south
	vec3(0, 1, 1),
	vec3(0, 0, 1),
	vec3(1, 0, 1),
	vec3(1, 1, 1),

	// north
	vec3(1, 1, 0),
	vec3(1, 0, 0),
	vec3(0, 0, 0),
	vec3(0, 1, 0),

	// west
	vec3(0, 1, 0),
	vec3(0, 0, 0),
	vec3(0, 0, 1),
	vec3(0, 1, 1),

	// east
	vec3(1, 1, 1),
	vec3(1, 0, 1),
	vec3(1, 0, 0),
	vec3(1, 1, 0)
);

out vec3 Normal;

void main()
{
	const vec3 size = vec3(12.f, 4.f, 12.f);
	//const vec3 positionOffset = vec3(0.f, -0.6f, 0.f);
	const vec3 pos = vec3(aPosition.x, 128.f, aPosition.y);

    gl_Position = camProj * camView * vec4((teste[gl_VertexID]  * size) + pos, 1.f);

    Normal = aNormal;
} 