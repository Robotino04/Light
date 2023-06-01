#include "Light/Image.hpp"

namespace Light{
std::ostream& operator << (std::ostream& os, Image const& image){
    // header
    os << "P6\n" << image.width << " " << image.height << " 255\n";
    for (int y=image.getHeight()-1; y>=0; y--){
        for (int x=0; x<image.width; x++){
            auto pixel = glm::clamp(image.at(x, y)*256.0f, glm::vec3(0), glm::vec3(255.999f));
            os << char(pixel.r) << char(pixel.g) << char(pixel.b);
        }
    }
    return os;
}


}