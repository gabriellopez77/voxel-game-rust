#version 460 core

layout (location = 0) in vec3 aVertex;
layout (location = 1) in vec2 aTexCoords;
layout (location = 2) in mat4 aModel;
layout (location = 6) in vec4 aTexCoordsInstance;
layout (location = 7) in vec4 aColor;

layout(std140, binding = 0) uniform shaderData {
    mat4 uiProj;
    float pixelScale;
    mat4 camProj;
    mat4 camView;
    mat4 camViewProj;
    mat4 viewNoTranslation;
};

uniform mat4 model;

out vec2 TexCoords;
out vec4 Color;

void main()
{
    gl_Position = camProj * viewNoTranslation * model * aModel * vec4(aVertex, 1.f);

    TexCoords = mix(aTexCoordsInstance.xy, aTexCoordsInstance.zw, aTexCoords);
    Color = aColor;
}