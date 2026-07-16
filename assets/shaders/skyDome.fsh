#version 460 core

#include "includes/globals.glsl"


layout(location = 0) in float Factor;

layout(location = 0) out vec4 out_color;

void main()
{
    const float DISTANCE = globalUbo.renderDistance / 8.f;

    out_color.rgb = mix(globalUbo.fogColor.rgb, globalUbo.skyColor.rgb, Factor);
    out_color.rgb = mix(globalUbo.skyColor.rgb, out_color.rgb, clamp(pow(1.f - Factor, DISTANCE), -1.f, 1.f));

    out_color.a = 1.f;
}
