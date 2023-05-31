#pragma once

#include "Light/Ray.hpp"
#include "Light/HittableObject.hpp"

#include "glm/glm.hpp"

namespace Light{

class Sphere : public HittableObject{
    public:
        Sphere(glm::vec3 pos, float radius): pos(pos), radius(radius){}
        bool hit(Ray const& ray, HitResult& hitResult) const override;

    public:
        glm::vec3 pos;
        float radius;
};

}