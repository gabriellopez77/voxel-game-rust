#version 460 core

layout (location = 0) in vec3 aVertex;

layout(std140, binding = 0) uniform shaderData {
    mat4 uiProj;
    float pixelScale;
    mat4 camProj;
    mat4 camView;
    mat4 camViewProj;
    mat4 viewNoTranslation;
};

uniform vec3 pos;

void main() {
    gl_Position = camViewProj * vec4(aVertex + pos, 1.f);
    gl_Position.z -= 0.001f;
}