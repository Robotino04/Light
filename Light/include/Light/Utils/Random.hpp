#pragma once

#include <random>

#include "glm/glm.hpp"
#include "glm/gtx/norm.hpp"

namespace Light::Utils{

namespace Detail{
    thread_local std::uniform_real_distribution<double> uniformDistribution01(0.0, 1.0);
    thread_local std::mt19937 rngGenerator;
}

// returns a random value in [0, 1]
inline float random(){
    return Detail::uniformDistribution01(Detail::rngGenerator);
}

glm::vec3 randomInUnitSphere(){
    glm::vec3 v;
    do {
        v = {
            random()*2.0-1.0,
            random()*2.0-1.0,
            random()*2.0-1.0,
        };
    } while(glm::length2(v) > 1);
    return v;
}

glm::vec3 randomOnUnitSphere(){
    return glm::normalize(randomInUnitSphere());
}

}