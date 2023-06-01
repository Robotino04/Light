#pragma once

#include "Light/Ray.hpp"
#include "Light/HittableObject.hpp"
#include "Light/Material.hpp"

namespace Light{

class SolidObject : public HittableObject{
    public:
        virtual bool hit(Ray const& ray, HitResult& hitResult) const = 0;
    
    public:
        Material material;
};

}