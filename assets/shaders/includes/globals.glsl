layout(std140, set = 0, binding = 1) readonly uniform sla {
    // ui
    mat4 uiProj;
    float pixelScale;

    // global camera
    mat4 camProj;
    mat4 camView;
    mat4 camViewProj;
    mat4 viewNoTranslation;

    // world
    vec3 skyColor;
    vec3 fogColor;
    vec3 lightColor;
    vec3 darknessColor;
    vec3 ambientColor;
    vec3 cloudsColor;
    float fogDistance;
    float fogDensity;
    int fogEnable;
    float renderDistance;
} globalUbo;
