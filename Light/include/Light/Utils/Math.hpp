#pragma once

namespace Light::Utils{

template <typename T>
T lerp(float x, T v0, T v1){
    return (1.0f-x)*v0 + x*v1;
}

}