#version 460 core

uniform sampler2D myTexture;

in vec3 Normal;
in vec2 TexCoords;

flat in int Shade;
in float AoLevel;

out vec4 outColor;

void main() {
    vec4 tex = texture(myTexture, TexCoords);

    if (tex.a < 0.1)
        discard;

    float shadeFace = 1.f;
    if (Shade == 1)
    {
        const float LIGHT_POWER = 0.6;
        const float AMBIENT_LIGHT_POWER = 0.4;

        const vec3 lightDir0 = vec3( 0.4f, 1.f,  0.6f);
        const vec3 lightDir1 = vec3(-0.4f, 1.f, -0.6f);

        float light0 = max(0.f, dot(lightDir0, Normal));
        float light1 = max(0.f, dot(lightDir1, Normal));
        shadeFace = min(1.0, (light0 + light1) * LIGHT_POWER + AMBIENT_LIGHT_POWER);
    }

    outColor = vec4(tex.rgb * (shadeFace * AoLevel), tex.a);
}
