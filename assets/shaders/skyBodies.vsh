#version 460 core

#include "includes/globals.glsl"


layout(location = 0) in vec3 aVertex;
layout(location = 1) in vec2 aTexCoords;
layout(location = 2) in mat4 aModel;
layout(location = 6) in vec4 aTexCoordsInstance;
layout(location = 7) in vec4 aColor;

layout(push_constant) uniform PushConstants {
    mat4 model;
    float alpha;
} push;


layout(location = 0) out vec2 TexCoords;
layout(location = 1) out vec4 Color;
layout(location = 2) out float Alpha;

void main()
{
    gl_Position = globalUbo.camProj * globalUbo.viewNoTranslation * push.model * aModel * vec4(aVertex, 1.f);

    TexCoords = mix(aTexCoordsInstance.xy, aTexCoordsInstance.zw, aTexCoords);
    Color = aColor;
    Alpha = push.alpha;
}
