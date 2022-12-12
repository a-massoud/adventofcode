cmake_minimum_required(VERSION 3.20)
project(AoCD12)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(AoCD12 src/main.cc
                      src/util.h
                      src/dijkstra.h
                      src/dijkstra.cc)
target_include_directories(AoCD12 PRIVATE src)
if(MSVC)
    target_compile_options(AoCD12 PRIVATE /W4)
else()
    target_compile_options(AoCD12 PRIVATE -Wall -Wextra -pedantic)
endif()
