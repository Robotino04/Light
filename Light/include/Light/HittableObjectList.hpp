#pragma once

#include "Light/Ray.hpp"
#include "Light/HittableObject.hpp"

#include "glm/glm.hpp"

#include <vector>
#include <memory>

namespace Light{

class HittableObjectList : public HittableObject{
    public:
        ~HittableObjectList();

        bool hit(Ray const& ray, HitResult& hitResult) const override;

    public:
        // manual memory management to avoid shared_ptr locking
        std::vector<HittableObject*> objects;
};

}