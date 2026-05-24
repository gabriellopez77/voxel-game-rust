#version 460 core

layout (location = 0) in vec3 aVertex;

layout(std140, binding = 0) uniform shaderData {
    mat4 uiProj;
    float pixelScale;
    mat4 camProj;
	mat4 camView;
	mat4 camViewProj;
	mat4 camViewNoTranslation;
};

out float Factor;

void main()
{
	gl_Position = camProj * camViewNoTranslation * vec4(aVertex, 1.f);

	Factor = clamp(aVertex.y, 0.f, 1.f);
}
