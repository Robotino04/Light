#include <iostream>
#include <string>
#include <fstream>
#include <chrono>
#include <thread>
#include <memory>
#include <execution>

#include "Light/Image.hpp"
#include "Light/Camera.hpp"
#include "Light/Sphere.hpp"
#include "Light/HittableObjectList.hpp"

#include "Light/Utils/Math.hpp"
#include "Light/Utils/Random.hpp"

#include "glm/glm.hpp"
#include "glm/gtc/random.hpp"

static constexpr float epsilon = 1e-8f;

glm::vec3 backgroundColor(Light::Ray const& ray) {
    glm::vec3 unit_direction = glm::normalize(ray.dir);
    float t = 0.5f*(unit_direction.y + 1.0f);
    
    return Light::Utils::lerp(t, glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.5, 0.7, 1.0));
}

glm::vec3 traceRay(Light::Ray const& ray, Light::HittableObject const& scene, int maxDepth, int depth=0){
    if (depth == maxDepth) return glm::vec3(0);
    
    Light::HitResult hitResult;
    if (scene.hit(ray, hitResult)){
        // return 0.5f*(hitResult.normal + 1.0f); // Normal shading

        Light::Ray newRay;
        newRay.origin = hitResult.hitPoint + hitResult.normal * epsilon;
        
        glm::vec3 pointOnUnitSphere = Light::Utils::randomOnUnitSphere();

        glm::vec3 specularTarget = hitResult.hitPoint + glm::reflect(ray.dir, hitResult.normal) + hitResult.hitObject->material.roughness * pointOnUnitSphere;
        glm::vec3 diffuseTarget = hitResult.hitPoint + hitResult.normal + pointOnUnitSphere;
        // in the range [1.5, 2.0[ tansition from specular to diffuse
        float scaledRoughness = glm::clamp(Light::Utils::remap(hitResult.hitObject->material.roughness, 1.5f, 1.9999f, 0.0f, 1.0f), 0.0f, 1.0f);
        
        glm::vec3 target;
        if (scaledRoughness == 0.0f)
            target = specularTarget;
        else if (scaledRoughness == 2.0f)
            target = diffuseTarget;
        else
            target = Light::Utils::random() > scaledRoughness ? specularTarget : diffuseTarget;

        newRay.dir = glm::normalize(target - hitResult.hitPoint);
        if (glm::dot(newRay.dir, hitResult.normal) < 0){
            newRay.dir = -newRay.dir;
        }
        
        return hitResult.hitObject->material.reflectivity * hitResult.hitObject->material.color * traceRay(newRay, scene, maxDepth, depth+1);
    }
    return backgroundColor(ray);
}

void render(Light::Image& image, Light::HittableObject const& scene, Light::Camera camera, int numSamplesPerPixel, int maxDepth, int progressBarDetail){
    std::cout << "[";
    for (int i=0; i<progressBarDetail; i++) std::cout << " ";
    std::cout << "]";
    for (int i=0; i<progressBarDetail+1; i++) std::cout << "\b";
    std::cout << std::flush;

    image.clear();
    std::atomic<int> progress = 0;
    std::thread ioThread([&](){
        int lastProgressStep = 0;
        while (true){
            int currentProgress = progress; // local copy to avoid locking the variable for the workers
            int progressStep = (currentProgress*progressBarDetail)/image.getHeight();
            while (progressStep > lastProgressStep){
                std::cout << "#";
                lastProgressStep++;
            }
            std::cout << std::flush;
            if (currentProgress == image.getHeight()) return;
            std::this_thread::sleep_for(std::chrono::milliseconds(20));

        }
    });
    
    std::vector<int> scanlines;
    for (int i=0; i<image.getHeight(); i++){
        scanlines.push_back(i);
    }

    std::for_each(std::execution::par_unseq, scanlines.begin(), scanlines.end(), [&](int j){
        for (int i=0; i<image.getWidth(); i++){
            glm::vec3 color(0);
            for (int sample=0; sample<numSamplesPerPixel; sample++){
                float u = float(i) + Light::Utils::random();
                float v = float(j) + Light::Utils::random();

                Light::Ray ray = camera.getViewRay(u, v);
                color += traceRay(ray, scene, maxDepth);
            }
            color /= float(numSamplesPerPixel);
            color = glm::sqrt(color);           // gamma correction for gamma=2

            image.at(i, j) = color;
        }

        progress++;
    });
    
    ioThread.join();
}

int main(){
    Light::Image image(1920, 1080);
    Light::Camera camera(image.getWidth(), image.getHeight());
    Light::HittableObjectList scene;
    const int numSamplesPerPixel = 50;
    const int maxDepth = 5;

    Light::Sphere* ground = new Light::Sphere(glm::vec3(0,-100.5,-1), 100);
    Light::Sphere* left = new Light::Sphere(glm::vec3(-1, 0, -1), 0.5);
    Light::Sphere* middle = new Light::Sphere(glm::vec3(0, 0, -1), 0.5);
    Light::Sphere* right = new Light::Sphere(glm::vec3(1, 0, -1), 0.5);
    ground->material.color = {0.8, 0.8, 0.0};
    ground->material.roughness = 1;
    ground->material.reflectivity = 1;
    left->material.color = {0.8, 0.8, 0.8};
    left->material.roughness = 0.3;
    middle->material.color = {0.7, 0.3, 0.3};
    middle->material.roughness = 2.0f;
    right->material.color = {0.8, 0.6, 0.2};
    right->material.roughness = 1.0;

    scene.objects.push_back(ground);
    scene.objects.push_back(left);
    scene.objects.push_back(middle);
    scene.objects.push_back(right);

    int imageNumber = 0;
    for (right->material.roughness = 0.0; right->material.roughness <= 2.00; right->material.roughness += 0.02){
        auto start = std::chrono::high_resolution_clock::now();
        render(image, scene, camera, numSamplesPerPixel, maxDepth, 50);
        auto end = std::chrono::high_resolution_clock::now();

        std::cout << "\n";
        std::cout << "Rendering took " << std::chrono::duration<float>(end-start).count() << "s.\n";


        std::ofstream file("test" + std::to_string(imageNumber) + "_" + std::to_string(right->material.roughness) + ".ppm");
        if (!file.is_open()){
            throw std::runtime_error("Unable to open file.");
        }
        file << image;
        file.close();
        imageNumber++;
    }


    return 0;
}