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

uniform vec3 pos;

out vec3 Normal;
out vec2 TexCoords;

flat out int Shade;
out float AoLevel;

void main() {
    gl_Position = camProj * camView * vec4(aVertex + pos, 1.f);

   	// flags and Ao level
	int flagsValue = int(aFlags);
    Shade = (flagsValue & SHADE_FLAG) >> 2;
    AoLevel = ((flagsValue & AO_LEVEL_FLAG) >> 0) / 3.f;

    Normal = aNormal;
    TexCoords = aTexCoords;
}
