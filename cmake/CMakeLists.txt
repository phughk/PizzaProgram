project(PizzaProject)
cmake_minimum_required(VERSION 3.5.1)
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

add_executable(pizza pizza.cpp)
target_link_libraries(pizza ${CONNAN_LIBS})
