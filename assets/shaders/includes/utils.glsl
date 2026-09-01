#include "globals.glsl"


float calculateFog(vec3 viewSpace)
{
    if (globalUbo.fogEnable == 0)
        return 1.0f;

    return exp(-pow(length(viewSpace) * globalUbo.fogDistance, globalUbo.fogDensity));
}

void applyFog(inout vec3 fragColor, float factor)
{
    fragColor = mix(globalUbo.fogColor.rgb, fragColor, factor);
}

vec3 calculateLightLevels(vec2 lightLevels)
{
    // mixes darknessColor and ambientColor with sky light strength as factor
    const vec3 DarknessAndAmbient = mix(globalUbo.darknessColor.rgb, globalUbo.ambientColor.rgb, smoothstep(0.f, 1.f, lightLevels.y));

    // mixes ambient color and Light block color with light block strength as factor
    return mix(DarknessAndAmbient, globalUbo.lightColor.rgb, smoothstep(0.f, 1.f, lightLevels.x));
}

vec2 getLightLevel(uint lightLevels)
{
   	uint lightValues = uint(lightLevels);

    uint blockValue = lightValues >> 4;
    uint skyValue = lightValues & 0xF;

	return vec2(float(blockValue), float(skyValue)) / 15.f;
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

mat4 buildTransform(vec3 position, vec3 scale, vec3 rotation)
{
    mat4 scaleMatrix = mat4(
        scale.x, 0.0,      0.0,      0.0,
        0.0,      scale.y, 0.0,      0.0,
        0.0,      0.0,      scale.z, 0.0,
        0.0,      0.0,      0.0,      1.0
    );

    vec3 rad = rotation;
    vec3 s = sin(rad);
    vec3 c = cos(rad);

    mat4 rotX = mat4(
        1.0, 0.0,  0.0,  0.0,
        0.0, c.x,  s.x,  0.0,
        0.0, -s.x, c.x,  0.0,
        0.0, 0.0,  0.0,  1.0
    );

    mat4 rotY = mat4(
        c.y,  0.0, -s.y, 0.0,
        0.0,  1.0, 0.0,  0.0,
        s.y,  0.0, c.y,  0.0,
        0.0,  0.0, 0.0,  1.0
    );

    mat4 rotZ = mat4(
        c.z,  s.z,  0.0, 0.0,
        -s.z, c.z,  0.0, 0.0,
        0.0,  0.0,  1.0, 0.0,
        0.0,  0.0,  0.0, 1.0
    );

    mat4 rotationMatrix = rotY * rotX * rotZ;

    mat4 translationMatrix = mat4(
        1.0,           0.0,           0.0,           0.0,
        0.0,           1.0,           0.0,           0.0,
        0.0,           0.0,           1.0,           0.0,
        position.x, position.y, position.z, 1.0
    );

    return translationMatrix * rotationMatrix * scaleMatrix;
}
