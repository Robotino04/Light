#include "Light/Image.hpp"

namespace Light{
std::ostream& operator << (std::ostream& os, Image const& image){
    // header
    os << "P3 " << image.width << " " << image.height << " 255\n";
    for (int y=image.getHeight()-1; y>=0; y--){
        for (int x=0; x<image.width; x++){
            auto pixel = image.at(x, y)*255.0f;
            os << " " << int(pixel.r) << " "<< int(pixel.g) << " "<< int(pixel.b);
        }
    }
    return os;
}


}