#include <iostream>
#include <string>
#include <fstream>
#include <chrono>
#include <thread>
#include <memory>

#include "Light/Image.hpp"
#include "Light/Camera.hpp"
#include "Light/Sphere.hpp"
#include "Light/HittableObjectList.hpp"

#include "Light/Utils/Math.hpp"
#include "Light/Utils/Random.hpp"

#include "glm/glm.hpp"

glm::vec3 backgroundColor(Light::Ray ray) {
    glm::vec3 unit_direction = glm::normalize(ray.dir);
    float t = 0.5*(unit_direction.y + 1.0);
    
    return Light::Utils::lerp(t, glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.5, 0.7, 1.0));
}

glm::vec3 perPixel(Light::Ray const& ray, Light::HittableObject const& scene){
    Light::HitResult hitResult;
    if (scene.hit(ray, hitResult)){
        return 0.5f*(hitResult.normal+ 1.0f);
    }
    return backgroundColor(ray);
}

int main(){
    Light::Image image(1920, 1080);
    Light::Camera camera(image.getWidth(), image.getHeight());
    Light::HittableObjectList scene;
    scene.objects.push_back(std::make_shared<Light::Sphere>(glm::vec3(0, 0, -1), 0.5));
    scene.objects.push_back(std::make_shared<Light::Sphere>(glm::vec3(0,-100.5,-1), 100));
    const int numSamplesPerPixel = 5;

    std::cout << "[";
    for (int i=0; i<50; i++) std::cout << " ";
    std::cout << "]";
    for (int i=0; i<51; i++) std::cout << "\b";
    std::cout << std::flush;

    image.clear();
    int progressbar=0;
    for (int j=0; j<image.getHeight(); j++){
        for (int i=0; i<image.getWidth(); i++){
            for (int sample=0; sample<numSamplesPerPixel; sample++){
                float u = float(i) + Light::Utils::random<float>();
                float v = float(j) + Light::Utils::random<float>();

                Light::Ray ray = camera.getViewRay(u, v);
                image.at(i, j) += perPixel(ray, scene);
            }
            image.at(i, j) /= float(numSamplesPerPixel);
        }

        // progress bar
        if (int((float(j+1)/float(image.getHeight()))*50) > progressbar){
            progressbar = int((float(j+1)/float(image.getHeight()))*50);
            std::cout << "#"  << std::flush;
        }
    }
    std::cout << "\n";

    std::ofstream file("test.ppm");
    if (!file.is_open()){
        throw std::runtime_error("Unable to open file.");
    }
    file << image;
    file.close();


    return 0;
}