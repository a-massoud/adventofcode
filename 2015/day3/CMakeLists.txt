cmake_minimum_required(VERSION 3.20)
project(aoc2015day3)

add_executable(aoc2015day3 src/main.c src/pointset.h src/pointset.c)

set_property(TARGET aoc2015day3 PROPERTY C_STANDARD 17)
set_property(TARGET aoc2015day3 PROPERTY C_STANDARD_REQUIRED ON)

target_include_directories(aoc2015day3 PRIVATE src)

if(MSVC)
    target_compile_options(aoc2015day3 PRIVATE /W4)
else()
    target_compile_options(aoc2015day3 PRIVATE -Wall -Wextra -pedantic)
endif()
