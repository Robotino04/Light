#include "Light/Camera.hpp"

#include "glm/gtc/matrix_transform.hpp"

namespace Light{
    Camera::Camera(int width, int height): width(width), height(height) {
        ratio = float(width)/float(height);
        projectionMatrix = glm::perspective(glm::radians(90.0f), ratio, 0.01f, 1000.0f);
        inverseProjectionMatrix = glm::inverse(projectionMatrix);
    }
    
    Ray Camera::getViewRay(float i, float j) const{
        Ray ray;
        glm::vec4 dir;
        dir.x = (i/float(width))*2-1;
        dir.y = (j/float(height))*2-1;
        dir.z = 1;
        dir.w = 1;

        ray.dir = inverseProjectionMatrix*dir;
        ray.dir = glm::normalize(ray.dir);
        ray.origin = {0,0,0};
        return ray;
    }
}
