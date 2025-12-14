cmake_minimum_required(VERSION 3.20)
project(aoc2015d8 LANGUAGES CXX)

add_executable(aoc2015d8 src/main.cc)

set_property(TARGET aoc2015d8 PROPERTY CXX_STANDARD 17)
set_property(TARGET aoc2015d8 PROPERTY CXX_STANDARD_REQUIRED ON)

if(MSVC)
    target_compile_options(aoc2015d8 PRIVATE /W4)
else()
    target_compile_options(aoc2015d8 PRIVATE -Wall -Wextra -pedantic)
endif()
