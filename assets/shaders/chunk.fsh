#version 460 core

#include "includes/globalTextures.glsl"
#include "includes/utils.glsl"


layout(push_constant) uniform PC {
    ChunksInstanceBDA bda;
    uint offsetIdx;
} ps;

layout(location = 0) in vec3 Normal;
layout(location = 1) in vec2 TexCoords;
layout(location = 2) flat in float Shade;
layout(location = 3) in float AoLevel;
layout(location = 4) flat in vec2 LightLevels;
layout(location = 5) in float FogFactor;
layout(location = 6) flat in uint DrawId;

layout(location = 0) out vec4 FragColor;

void main()
{
    vec4 tex = bindlessTexture(WORLD_TEXTURE_IDX, TexCoords);

    if (tex.a < 0.1)
        discard;

    const vec3 lightColor = calculateLightColor(LightLevels);

    const float fadeInEffect = ps.bda.data[DrawId + ps.offsetIdx].fadeInEffect;

    FragColor = vec4(tex.rgb * lightColor * (Shade * AoLevel), tex.a * fadeInEffect);
    applyFog(FragColor.rgb, FogFactor);
}
