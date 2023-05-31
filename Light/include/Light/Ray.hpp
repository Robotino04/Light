#pragma once

#include "glm/glm.hpp"

namespace Light{

struct Ray{
    glm::vec3 origin;
    glm::vec3 dir;

    glm::vec3 at(float t) const{
        return origin + dir*t;
    }
};

}