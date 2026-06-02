#version 460 core

layout (std140, binding = 1) uniform worldData {
    // fog
    vec3 skyColor;
    vec3 fogColor;
    vec3 lightColor;
    vec3 darknessColor;
    vec3 ambientColor;
    vec3 cloudsColor;
    float fogDistance;
    float fogDensity;
    int fogEnable;
};

in float Factor;
out vec4 out_color;

void main()
{
    out_color.rgb = mix(fogColor.xyz, skyColor.xyz, Factor);
    out_color.rgb = mix(skyColor.xyz, out_color.rgb, 1.f - Factor);

    out_color.a = 1.f;
}
