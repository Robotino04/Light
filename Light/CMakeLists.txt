cmake_minimum_required(VERSION 3.0)

file(GLOB_RECURSE LIGHT_SRC "*.cpp" "*.hpp" "*.tpp")

add_executable(Light ${LIGHT_SRC})

target_include_directories(Light PUBLIC "include/")
target_compile_features(Light PUBLIC cxx_std_20)

target_link_libraries(Light PUBLIC ANSI glm tbb)