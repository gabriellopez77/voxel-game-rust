#version 460 core

#include "includes/globals.glsl"


layout(location = 0) in vec3 aVertex;

layout(location = 0) out float Factor;

void main()
{
	gl_Position = globalUbo.camProj * globalUbo.viewNoTranslation * vec4(aVertex, 1.f);

	Factor = clamp(aVertex.y, 0.f, 1.f);
}
