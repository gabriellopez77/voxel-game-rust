#version 460 core

layout (location = 0) in vec4 aVertex;
layout (location = 1) in vec2 aPosition;
layout (location = 2) in vec2 aSize;
layout (location = 3) in vec4 aInstanceTexCoords;
layout (location = 4) in vec4 aColor;

layout(std140, binding = 0) uniform shaderData {
    mat4 uiProj;
    float pixelScale;
    mat4 camProj;
	mat4 camView;
	mat4 camViewProj;
	mat4 viewNoTranslation;
};

out vec2 TexCoords;
out vec4 Color;

void main() {
    gl_Position = uiProj * vec4(aVertex.xy * (aSize * pixelScale) + (aPosition * pixelScale), 0.f, 1.f);

    TexCoords = mix(aInstanceTexCoords.xy, aInstanceTexCoords.zw, aVertex.zw);
    Color = aColor / 255.f;
}
