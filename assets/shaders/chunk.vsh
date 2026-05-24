#version 460 core

layout (location = 0) in vec3 aVertex;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

layout(std140, binding = 0) uniform shaderData {
    mat4 uiProj;
    float pixelScale;
    mat4 camProj;
	mat4 camView;
	mat4 camViewProj;
	mat4 viewNoTranslation;
};

uniform vec3 pos;

out vec3 Normal;
out vec2 TexCoords;

void main() {
    gl_Position = camProj * camView * vec4(aVertex + pos, 1.f);

    Normal = aNormal;
    TexCoords = aTexCoords;
}
