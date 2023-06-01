#pragma once

#include <cstdlib>

namespace Light::Utils{

template<typename T>
T random(){
    return T(rand()) / T(RAND_MAX);
}

}