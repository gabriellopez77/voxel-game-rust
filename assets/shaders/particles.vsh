#version 460 core

#include "includes/globals.glsl"


layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec2 aTexCoords;
layout(location = 2) in vec3 aPosition;
layout(location = 3) in vec3 aScale;
layout(location = 4) in vec3 aRotation;
layout(location = 5) in vec4 aInstanceTexCoords;
layout(location = 6) in uint aTextureIdx;


layout(location = 0) out vec2 TexCoords;
layout(location = 1) out uint TextureIdx;

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

void main()
{
    mat4 matrix = buildTransform(aPosition, aScale, aRotation);

    gl_Position = globalUbo.camViewProj * matrix * vec4(aVertex, 1.f);

    TexCoords = mix(aInstanceTexCoords.xy, aInstanceTexCoords.zw, aTexCoords);
    TextureIdx = aTextureIdx;
}
