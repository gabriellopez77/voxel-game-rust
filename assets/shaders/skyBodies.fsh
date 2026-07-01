#version 460 core

#include "includes/globalTextures.glsl"


layout(location = 0) in vec2 TexCoords;
layout(location = 1) in vec4 Color;
layout(location = 2) in float Alpha;

layout(location = 0) out vec4 FragColor;

void main()
{
    bool isStar = bool(Color.a);

    if (isStar)
    {
        FragColor = vec4(Color.rgb, Alpha);
    }
    else
    {
        vec4 tex = bindlessTexture(SKY_BODIES_TEXTURE_IDX, TexCoords);

        if (tex.a < 0.1f)
            discard;

        FragColor = tex;
    }
}
