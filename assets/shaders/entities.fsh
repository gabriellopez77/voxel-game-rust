#version 460 core

#include "includes/globalTextures.glsl"
#include "includes/utils.glsl"


layout(location = 0) in vec3 Normal;
layout(location = 1) in vec2 TexCoords;
layout(location = 2) in flat uint TextureIdx;
layout(location = 3) in vec4 OverlayColor;
layout(location = 4) in float FogFactor;

layout(location = 0) out vec4 FragColor;

void main()
{
    vec4 tex = bindlessTexture(WORLD_TEXTURE_IDX, TexCoords);

    if (tex.a < 0.1)
        discard;

    const float shadeFace = calculateShading(Normal);

    FragColor = vec4(tex.rgb * shadeFace, tex.a);
    FragColor.rgb = mix(FragColor.rgb, OverlayColor.rgb, OverlayColor.a);

    applyFog(FragColor.rgb, FogFactor);
}
