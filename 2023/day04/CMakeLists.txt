cmake_minimum_required(VERSION 3.20)
project(aoc2023d4 LANGUAGES CXX)

add_executable(aoc2023d4 src/main.cc
                         src/card.h
                         src/card.cc)
target_include_directories(aoc2023d4 PRIVATE src)
set_target_properties(aoc2023d4 PROPERTIES CXX_STANDARD 20 CXX_STANDARD_REQUIRED ON)
if(MSVC)
    target_compile_options(aoc2023d4 PRIVATE /W4)
else()
    target_compile_options(aoc2023d4 PRIVATE -Wall -Wextra -pedantic)
endif()
