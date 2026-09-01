#version 460 core

#include "includes/globals.glsl"
#include "includes/utils.glsl"


layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec3 aPacked1; // tex coords[x, y], face id[z]
layout(location = 3) in vec4 aFaceTexCoords[6];
layout(location = 9) in uvec2 aPacked2; // overlay color[x], texture idx[y]
layout(location = 10) in mat4 aLocalMatrix;

layout(location = 0) out vec3 Normal;
layout(location = 1) out vec2 TexCoords;
layout(location = 2) out uint TextureIdx;
layout(location = 3) out vec4 OverlayColor;
layout(location = 4) out float FogFactor;

void main()
{
   	vec4 viewSpace = globalUbo.camView * aLocalMatrix * vec4(aVertex, 1.f);
    gl_Position = globalUbo.camProj * viewSpace;

    vec4 faceTexCoords = aFaceTexCoords[uint(aPacked1.z)];

    vec4 color = vec4(
        float(aPacked2.x & 0xFF),
        float((aPacked2.x >> 8) & 0xFF),
        float((aPacked2.x >> 16) & 0xFF),
        float((aPacked2.x >> 24) & 0xFF)
    );

    Normal = aNormal;
    TexCoords = mix(faceTexCoords.xy, faceTexCoords.zw, aPacked1.xy);
    TextureIdx = uint(aPacked2.y);
    OverlayColor = color / 255.0;
    FogFactor = calculateFog(viewSpace.xyz);
}
