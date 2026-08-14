#extension GL_EXT_nonuniform_qualifier : require

#define WORLD_TEXTURE_IDX (0)
#define UI_SPRITES_TEXTURE_IDX (1)
#define UI_FONTS_TEXTURE_IDX (2)
#define SKY_BODIES_TEXTURE_IDX (3)


layout(set = 0, binding = 0) uniform sampler2D GLOBAL_TEXTURES[];

vec4 bindlessTexture(uint idx, vec2 uv)
{
    return texture(GLOBAL_TEXTURES[nonuniformEXT(idx)], uv);
}
