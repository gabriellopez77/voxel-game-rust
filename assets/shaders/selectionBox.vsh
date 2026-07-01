#version 460 core

#include "includes/globals.glsl"


layout(location = 0) in ivec3 aVertex;

layout(push_constant) uniform PushConstants {
    vec3 pos;
} push;

void main() {
    gl_Position = globalUbo.camViewProj * vec4(aVertex + push.pos, 1.f);
    gl_Position.z -= 0.001f;
}
