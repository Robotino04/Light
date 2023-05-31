#include "Light/Sphere.hpp"

#include "glm/gtx/norm.hpp"

namespace Light{
bool Sphere::hit(Ray const& ray, HitResult& hitResult) const{
    glm::vec3 oc = ray.origin - pos;
    auto a = glm::length2(ray.dir);
    auto half_b = glm::dot(oc, ray.dir);
    auto c = glm::length2(oc) - radius*radius;
    auto discriminant = half_b*half_b - a*c;
    if (discriminant < 0) return false;

    // 1 or 2 intersections
    float t = (-half_b - glm::sqrt(discriminant)) / a;
    if (t > 0 && t < hitResult.t){
        hitResult.t = t;
        hitResult.hitPoint = ray.at(hitResult.t);
        hitResult.normal = glm::normalize(hitResult.hitPoint - pos);
        return true;
    }
    return false;
}

}