#version 460 core

#include "../includes/globalTextures.glsl"


layout(location = 0) in vec2 TexCoords;
layout(location = 1) in vec4 Color;
layout(location = 2) in flat uint TextureIdx;

layout(location = 0) out vec4 FragColor;

void main()
{
    if (TextureIdx == 255)
    {
        FragColor = Color;
    }
    else
    {
        vec4 tex = bindlessTexture(TextureIdx, TexCoords);

        if (tex.a < 0.1f)
            discard;

        FragColor = tex * Color;
    }
}
