#version 460 core

uniform sampler2D myTexture;

in vec2 TexCoords;
in vec3 Color;

out vec4 FragColor;

void main()
{
    vec4 tex = texture(myTexture, TexCoords);

    if (tex.a < 0.1f)
        discard;

    FragColor = vec4(tex.rgb * Color, 1.f);
}