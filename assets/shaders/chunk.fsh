#version 460 core

#include "includes/globals.glsl"
#include "includes/globalTextures.glsl"
#include "includes/utils.glsl"


//layout(push_constant) uniform PushConstants {
//    float value;
//} fadeInEffect;

layout(location = 0) in vec3 Normal;
layout(location = 1) in vec2 TexCoords;

layout(location = 2) flat in int Shade;
layout(location = 3) in float AoLevel;

layout(location = 4) in float FogFactor;


layout(location = 0) out vec4 FragColor;

void main() {
    vec4 tex = bindlessTexture(WORLD_TEXTURE_IDX, TexCoords);

    if (tex.a < 0.1)
        discard;

    // face shading
    const float shadeFace = Shade == 1 ? calculateShading(Normal) : 1.0f;

    FragColor = vec4(tex.rgb * (shadeFace * AoLevel), tex.a);

    applyFog(FragColor.xyz, FogFactor);
}
