#pragma once

#include "Light/Ray.hpp"
#include "Light/HittableObject.hpp"

#include "glm/glm.hpp"

#include <vector>
#include <memory>

namespace Light{

class HittableObjectList : public HittableObject{
    public:
        bool hit(Ray const& ray, HitResult& hitResult) const override;

    public:
        std::vector<std::shared_ptr<HittableObject>> objects;
};

}