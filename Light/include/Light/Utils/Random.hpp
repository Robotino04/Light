#pragma once

#include <random>
#include <limits>
#include <array>

#include "glm/glm.hpp"
#include "glm/gtx/norm.hpp"

namespace Light::Utils{

namespace Detail{
    thread_local std::uniform_real_distribution<double> uniformDistribution01(0.0, 1.0);
    thread_local std::mt19937 rngGenerator;

    // https://stackoverflow.com/a/1227137
    /* return 32 bit random number */
    uint32_t WELLRNG512(void){

        /* initialize state to random bits */
        static thread_local std::array<uint32_t, 16> state = [](){
            std::array<uint32_t, 16> state;
            for (int i=0; i<16; i++){
                state.at(i) = std::rand();
            }
            return state;
        }();

        /* init should also reset this to 0 */
        static thread_local uint32_t index = 0;

        uint32_t a, b, c, d;
        a = state[index];
        c = state[(index+13)&15];
        b = a^c^(a<<16)^(c<<15);
        c = state[(index+9)&15];
        c ^= (c>>11);
        a = state[index] = b^c;
        d = a^((a<<5)&0xDA442D24UL);
        index = (index + 15)&15;
        a = state[index];
        state[index] = a^b^d^(a<<2)^(b<<18)^(c<<28);
        return state[index];
    }
}

// returns a random value in [0, 1]
inline float random(){
    // return Detail::uniformDistribution01(Detail::rngGenerator);
    return float(Detail::WELLRNG512())/float(std::numeric_limits<uint32_t>::max());
}

glm::vec3 randomInUnitSphere(){
    glm::vec3 v;
    do {
        v = {
            random()*2.0-1.0,
            random()*2.0-1.0,
            random()*2.0-1.0,
        };
    } while(glm::length2(v) > 1);
    return v;
}

glm::vec3 randomOnUnitSphere(){
    return glm::normalize(randomInUnitSphere());
}

}