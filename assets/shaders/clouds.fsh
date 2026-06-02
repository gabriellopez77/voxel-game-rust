#version 460 core


layout (std140, binding = 1) uniform worldData {
    // fog
    vec3 skyColor;
    vec3 fogColor;
    vec3 lightColor;
    vec3 darknessColor;
    vec3 ambientColor;
    vec3 cloudsColor;
    float fogDistance;
    float fogDensity;
    int fogEnable;
};

in vec3 Normal;

in float FogFactor;


out vec4 FragColor;

void main()
{
	const float LIGHT_POWER = 0.6;
    const float AMBIENT_LIGHT_POWER = 0.4;

    float light0 = max(0.f, dot(vec3( 0.6f, 1.f,  0.8f), Normal));
    float light1 = max(0.f, dot(vec3(-0.6f, -0.4f, -0.8f), Normal));
    float shadeFace = min(1.0, (light0 + light1) * LIGHT_POWER + AMBIENT_LIGHT_POWER);
    
    FragColor = vec4(cloudsColor * shadeFace, 0.8f);

    // fog
    FragColor.rgb = mix(fogColor.xyz, FragColor.rgb, FogFactor);
}