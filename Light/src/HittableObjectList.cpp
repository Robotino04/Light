#include "Light/HittableObjectList.hpp"

namespace Light{

HittableObjectList::~HittableObjectList(){
    for (auto object : objects){
        delete object;
    }
}

bool HittableObjectList::hit(Ray const& ray, HitResult& hitResult) const{
    bool hitAny = false;
    for (auto obj : objects){
        hitAny |= obj->hit(ray, hitResult);
    }
    return hitAny;

}

}