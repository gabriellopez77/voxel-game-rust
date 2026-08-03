#version 460 core

#include "includes/globalTextures.glsl"
#include "includes/utils.glsl"


layout(location = 0) in vec3 Normal;
layout(location = 1) in vec2 TexCoords;

layout(location = 0) out vec4 FragColor;

void main() {
    vec4 tex = bindlessTexture(WORLD_TEXTURE_IDX, TexCoords);

    if (tex.a < 0.1f)
        discard;

    float shadeFace = calculateShading(Normal);

    FragColor = vec4(tex.rgb * shadeFace, tex.a);
}
