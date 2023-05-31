#pragma once

#include "Light/Ray.hpp"
#include "Light/HitResult.hpp"

namespace Light{

class HittableObject{
    public:
        virtual bool hit(Ray const& ray, HitResult& hitResult) const = 0;
};

}