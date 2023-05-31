#include <iostream>
#include <string>
#include <fstream>
#include <chrono>
#include <thread>

#include "Light/Image.hpp"

#include "glm/glm.hpp"

int main(){
    Light::Image image(256, 256);

    std::cout << "[";
    for (int i=0; i<50; i++) std::cout << " ";
    std::cout << "]";
    for (int i=0; i<51; i++) std::cout << "\b";
    std::cout << std::flush;

    int progressbar=0;
    for (int j=0; j<image.getHeight(); j++){
        for (int i=0; i<image.getWidth(); i++){
            auto r = double(i) / (image.getWidth()-1);
            auto g = double(j) / (image.getHeight()-1);
            auto b = 0.25;
            image.at(i, j) = {r, g, b};
        }
        if (int((float(j+1)/float(image.getHeight()))*50) > progressbar){
            progressbar = int((float(j+1)/float(image.getHeight()))*50);
            std::cout << "#" << std::flush;
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