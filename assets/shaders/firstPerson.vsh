#version 460 core

#include "includes/globals.glsl"


layout(push_constant) uniform PushConstants {
    mat4 model;
} push;

layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoords;

layout(location = 0) out vec3 Normal;
layout(location = 1) out vec2 TexCoords;

void main()
{
    gl_Position = globalUbo.firstPersonProj * push.model * vec4(aVertex, 1.0);

    Normal = aNormal;
    TexCoords = aTexCoords;
}
