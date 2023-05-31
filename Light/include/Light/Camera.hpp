#pragma once

#include "glm/glm.hpp"

#include "Light/Ray.hpp"

namespace Light{

class Camera{
    public:
        Camera(int width, int height);

        Ray getViewRay(int i, int j) const;

    private:
        glm::mat4x4 projectionMatrix;
        glm::mat4x4 inverseProjectionMatrix;
        int width, height;
        float ratio;
};

}