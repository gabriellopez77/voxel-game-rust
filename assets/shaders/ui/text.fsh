#version 460 core

#include "../includes/globalTextures.glsl"


layout(location = 0) in vec2 TexCoords;
layout(location = 1) in vec3 Color;

layout(location = 0) out vec4 FragColor;

void main()
{
    vec4 tex = bindlessTexture(UI_FONTS_TEXTURE_IDX, TexCoords);

    if (tex.a < 0.1f)
        discard;

    FragColor = vec4(tex.rgb * Color, 1.f);
}
