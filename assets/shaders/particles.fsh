#version 460 core

#include "includes/globalTextures.glsl"
#include "includes/utils.glsl"


layout(location = 0) in vec2 TexCoords;
layout(location = 1) in flat uint TextureIdx;
layout(location = 2) in float FogFactor;

layout(location = 0) out vec4 FragColor;

void main() {
    vec4 tex = bindlessTexture(TextureIdx, TexCoords);

    if (tex.a < 0.1f)
        discard;

    FragColor = tex;

    applyFog(FragColor.xyz, FogFactor);
}
