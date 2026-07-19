#version 460 core

#include "includes/globals.glsl"
#include "includes/utils.glsl"


layout(location = 0) in vec3 Normal;

layout(location = 1) in float FogFactor;


layout(location = 0) out vec4 FragColor;

void main()
{
    float shading = calculateShading2(Normal);

    FragColor = vec4(globalUbo.cloudsColor.rgb * shading, 0.8f);
    applyFog(FragColor.rgb, FogFactor);
}
