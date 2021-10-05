#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct SpotLight {
    vec3 position;
    vec3 direction;
    float cutOff;
    float outerCutOff;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

struct PointLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

struct DirectionLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 oNormal;
in vec3 fragPos;
in vec2 texCoords;
out vec4 FragColor;

uniform vec3 lightColor;
uniform vec3 objectColor;
//uniform vec3 lightPos;
uniform vec3 viewPos;
uniform bool toggleTorchLight;

uniform Material material;
uniform SpotLight spotLight;
uniform DirectionLight dirLight;
#define NR_POINT_LIGHTS 1
uniform PointLight pointLights[NR_POINT_LIGHTS];
//uniform PointLight pointLight;

vec3 calcSpotLight(SpotLight light, vec3 normal, vec3 aFragPos, vec3 viewDir) {
    vec3 lightDir = normalize(light.position - aFragPos);
    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon   = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

    float distance    = length(light.position - aFragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance +
    light.quadratic * (distance * distance));

    vec3 ambient = light.ambient * vec3(texture(material.diffuse, texCoords));


    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, texCoords));


    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * vec3(texture(material.specular, texCoords));

    ambient*= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    diffuse  *= intensity;
    specular *= intensity;
    return (ambient + diffuse + specular);
}

vec3 calcPointLight(PointLight light, vec3 normal, vec3 aFragPos, vec3 viewDir) {
    float distance    = length(light.position - aFragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance +
    light.quadratic * (distance * distance));

    vec3 ambient = light.ambient * vec3(texture(material.diffuse, texCoords));
    ambient*= attenuation;

    vec3 lightDir = normalize(light.position - aFragPos);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, texCoords));
    diffuse *= attenuation;

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * vec3(texture(material.specular, texCoords));
    specular *= attenuation;

    return (ambient + diffuse + specular);
}

vec3 calcDirLight(DirectionLight light, vec3 normal, vec3 aFragPos, vec3 viewDir) {

    vec3 ambient = light.ambient * vec3(texture(material.diffuse, texCoords));

    vec3 lightDir = normalize(light.position - aFragPos);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, texCoords));

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * vec3(texture(material.specular, texCoords));

    return (ambient + diffuse + specular);
}

void main()
{
    vec3 norm = normalize(oNormal);
    vec3 viewDir = normalize(viewPos - fragPos);
    // phase 1: Directional lighting
    vec3 result = calcDirLight(dirLight, norm, fragPos, viewDir);
    // phase 2: Point lights
    for (int i = 0; i < NR_POINT_LIGHTS; i++)
    result += calcPointLight(pointLights[i], norm, fragPos, viewDir);
    //    result += calcPointLight(pointLight, norm, fragPos, viewDir);
    // phase 3: Spot light
    if (toggleTorchLight)
    result += calcSpotLight(spotLight, norm, fragPos, viewDir);

    FragColor = vec4(result, 1.0);

}

