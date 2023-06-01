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

glm::vec3 backgroundColor(Light::Ray const& ray) {
    glm::vec3 unit_direction = glm::normalize(ray.dir);
    float t = 0.5f*(unit_direction.y + 1.0f);
    
    return Light::Utils::lerp(t, glm::vec3(1.0, 1.0, 1.0), glm::vec3(0.5, 0.7, 1.0));
}

glm::vec3 perPixel(Light::Ray const& ray, Light::HittableObject const& scene){
    Light::HitResult hitResult;
    if (scene.hit(ray, hitResult)){
        return 0.5f*(hitResult.normal + 1.0f);
    }
    return backgroundColor(ray);
}

int main(){
    Light::Image image(1920, 1080);
    Light::Camera camera(image.getWidth(), image.getHeight());
    Light::HittableObjectList scene;
    scene.objects.push_back(new Light::Sphere(glm::vec3(1, 0, -1), 0.5));
    scene.objects.push_back(new Light::Sphere(glm::vec3(0,-100.5,-1), 100));
    const int numSamplesPerPixel = 50;

    const int progressBarDetail = 50;
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
    auto start = std::chrono::high_resolution_clock::now();

    std::for_each(std::execution::par_unseq, scanlines.begin(), scanlines.end(), [&](int j){
        for (int i=0; i<image.getWidth(); i++){
            glm::vec3 color(0);
            for (int sample=0; sample<numSamplesPerPixel; sample++){
                float u = float(i) + Light::Utils::random();
                float v = float(j) + Light::Utils::random();

                Light::Ray ray = camera.getViewRay(u, v);
                color += perPixel(ray, scene);
            }
            image.at(i, j) = color / float(numSamplesPerPixel);
        }

        progress++;
    });
    
    auto end = std::chrono::high_resolution_clock::now();
    ioThread.join();

    std::cout << "\n";
    std::cout << "Rendering took " << std::chrono::duration<float>(end-start).count() << "s.\n";


    std::ofstream file("test.ppm");
    if (!file.is_open()){
        throw std::runtime_error("Unable to open file.");
    }
    file << image;
    file.close();


    return 0;
}