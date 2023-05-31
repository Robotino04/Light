#include "Light/HittableObjectList.hpp"

namespace Light{

bool HittableObjectList::hit(Ray const& ray, HitResult& hitResult) const{
    bool hitAny = false;
    for (auto obj : objects){
        hitAny |= obj->hit(ray, hitResult);
    }
    return hitAny;

}

}