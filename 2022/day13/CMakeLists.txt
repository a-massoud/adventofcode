cmake_minimum_required(VERSION 3.20)
project(AoCD13)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(AoCD13 src/main.cc)
target_include_directories(AoCD13 PRIVATE src)
if(MSVC)
    target_compile_options(AoCD13 PRIVATE /W4)
else()
    target_compile_options(AoCD13 PRIVATE -Wall -Wextra -pedantic)
endif()
