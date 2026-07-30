#version 460 core

#include "includes/globals.glsl"


layout(location = 0) in ivec3 aVertex;

layout(push_constant) uniform PushConstants {
    vec3 pos;
    vec3 size;
} push;

void main() {
    vec3 vertexF = vec3(float(aVertex.x), float(aVertex.y), float(aVertex.z));

    gl_Position = globalUbo.camViewProj * vec4((vertexF * push.size) + push.pos, 1.f);
    gl_Position.z -= 0.001f;
}
