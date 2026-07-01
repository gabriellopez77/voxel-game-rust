#include "globals.glsl"

float calculateFog(vec3 viewSpace)
{
    if (!bool(globalUbo.fogEnable))
        return 1.0f;

    float distanceFromCamera = length(viewSpace);
    return exp(-pow(distanceFromCamera * globalUbo.fogDistance, globalUbo.fogDensity));
}

void applyFog(inout vec3 fragColor, float factor)
{
    fragColor = mix(globalUbo.fogColor, fragColor, factor);
}

float calculateShading(vec3 normal)
{
    const float LIGHT_POWER = 0.6;
    const float AMBIENT_LIGHT_POWER = 0.4;

    const vec3 lightDir0 = vec3( 0.4f, 1.f,  0.6f);
    const vec3 lightDir1 = vec3(-0.4f, 1.f, -0.6f);

    float light0 = max(0.f, dot(lightDir0, normal));
    float light1 = max(0.f, dot(lightDir1, normal));
    return min(1.f, (light0 + light1) * LIGHT_POWER + AMBIENT_LIGHT_POWER);
}

float calculateShading2(vec3 normal)
{
    const float LIGHT_POWER = 0.6;
    const float AMBIENT_LIGHT_POWER = 0.4;

    const vec3 lightDir0 = vec3( 0.6f, 1.f,  0.8f);
    const vec3 lightDir1 = vec3(-0.6f, -0.4f, -0.8f);

    float light0 = max(0.f, dot(lightDir0, normal));
    float light1 = max(0.f, dot(lightDir1, normal));
    return min(1.f, (light0 + light1) * LIGHT_POWER + AMBIENT_LIGHT_POWER);
}