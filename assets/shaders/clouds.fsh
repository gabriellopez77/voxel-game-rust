#version 460 core

in vec3 Normal;

out vec4 FragColor;

void main()
{
	const float LIGHT_POWER = 0.6;
    const float AMBIENT_LIGHT_POWER = 0.4;

    const vec3 lightDir0 = vec3( 0.6f, 1.f,  0.8f);
    const vec3 lightDir1 = vec3(-0.6f, -0.4f, -0.8f);

    float light0 = max(0.f, dot(lightDir0, Normal));
    float light1 = max(0.f, dot(lightDir1, Normal));
    float shadeFace = min(1.0, (light0 + light1) * LIGHT_POWER + AMBIENT_LIGHT_POWER);
    
    FragColor = vec4(vec3(shadeFace), 0.8f);
}