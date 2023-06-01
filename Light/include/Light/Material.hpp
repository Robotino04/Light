#pragma once

#include "glm/glm.hpp"

namespace Light {

struct Material {
    glm::vec3 color;            // Base color of the material
    float reflectivity;         // Reflectivity of the material (0.0 - 1.0)
    float transparency;         // Transparency of the material (0.0 - 1.0)
    float roughness;            // Roughness of the material (0.0 - 1.0)
    float refractiveIndex;      // Index of refraction for transparent materials
    glm::vec3 emissiveColor;    // Emissive color of the material
};

}