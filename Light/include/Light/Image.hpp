#pragma once

#include "glm/glm.hpp"

#include <vector>
#include <ostream>

namespace Light{

class Image{
    public:
        Image(int w, int h): width(w), height(h){
            pixels.resize(w*h);
        }

        glm::vec3& at(int x, int y){
            assert(x >= 0 && x < width);
            assert(y >= 0 && y < height);

            return pixels.at(x + y*width);
        }

        glm::vec3 at(int x, int y) const{
            assert(x >= 0 && x < width);
            assert(y >= 0 && y < height);

            return pixels.at(x + y*width);
        }

        int getWidth() const { return width; }
        int getHeight() const { return height; }

        void clear(){
            std::fill(pixels.begin(), pixels.end(), glm::vec3(0,0,0));
        }

        friend std::ostream& operator << (std::ostream& os, Image const& image);
    private:
        int width, height;
        std::vector<glm::vec3> pixels;
};

std::ostream& operator << (std::ostream& os, Image const& image);


}