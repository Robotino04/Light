#include "Light/HittableObjectList.hpp"

namespace Light{

HittableObjectList::~HittableObjectList(){
    for (auto obj : objects){
        delete obj;
    }
}
bool HittableObjectList::hit(Ray const& ray, HitResult& hitResult) const{
    bool hitAny = false;
    for (auto obj : objects){
        hitAny = obj->hit(ray, hitResult) || hitAny;
    }
    return hitAny;

}

}