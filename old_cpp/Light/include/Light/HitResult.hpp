#pragma once

#include "glm/glm.hpp"

#include <limits>

namespace Light{

class SolidObject;
struct HitResult{
    float t = std::numeric_limits<float>::infinity();
    glm::vec3 hitPoint;
    glm::vec3 normal;
    const SolidObject* hitObject = nullptr;
};

}