#version 460 core

uniform sampler2D myTexture;

uniform float alpha;

in vec2 TexCoords;
in vec4 Color;

out vec4 FragColor;

void main()
{
    bool isStar = bool(Color.a);

    if (isStar)
    {
        FragColor = vec4(Color.rgb, alpha);
    }
    else
    {
        vec4 tex = texture(myTexture, TexCoords);

        if (tex.a < 0.1f)
            discard;

        FragColor = vec4(tex);
    }
}
