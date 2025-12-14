cmake_minimum_required(VERSION 3.20)
project(aoc2015d7)

add_executable(aoc2015d7 src/main.cc
                         src/circuit.hh
                         src/circuit.cc)

set_property(TARGET aoc2015d7 PROPERTY CXX_STANDARD 17)
set_property(TARGET aoc2015d7 PROPERTY CXX_STANDARD_REQUIRED ON)

if(MSVC)
    target_compile_options(aoc2015d7 PRIVATE /W4)
else()
    target_compile_options(aoc2015d7 PRIVATE -Wall -Wextra -pedantic)
endif()
