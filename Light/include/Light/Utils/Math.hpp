#pragma once

namespace Light::Utils{

template <typename T>
T lerp(float x, T v0, T v1){
    return (1.0f-x)*v0 + x*v1;
}

template <typename T>
T remap(T x, T start_low, T start_high, T end_low, T end_high){
    return (x-start_low)/(start_high-start_low)*(end_high - end_low) + end_low;
}

}