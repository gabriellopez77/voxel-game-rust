#version 460 core

uniform sampler2D myTexture;

in vec4 Color;
in vec2 TexCoords;

out vec4 outColor;

void main() {
    vec4 tex = texture(myTexture, TexCoords);

    if (tex.a < 0.1f)
        discard;

    outColor = vec4(tex.rgb, 1.f);
}