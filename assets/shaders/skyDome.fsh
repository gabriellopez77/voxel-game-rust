#version 460 core

layout (std140, binding = 1) uniform fogSettings {
    vec3 skyColor;
    vec3 fogColor;
    vec3 lightColor;
    vec3 darknessColor;
    vec3 ambientColor;
    float fogDistance;
    float fogDensity;
    int fogEnable;
};

in float Factor;
out vec4 out_color;

void main()
{
    out_color.rgb = mix(fogColor, skyColor, Factor);
    out_color.rgb = mix(skyColor, out_color.rgb, 1.f - Factor);

    out_color.a = 1.f;
}
