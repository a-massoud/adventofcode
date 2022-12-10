cmake_minimum_required(VERSION 3.20)
project(AoCD8)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

add_executable(AoCD8 src/main.c
                     src/point_set.h
                     src/point_set.c)
target_include_directories(AoCD8 PRIVATE src)
if(MSVC)
    target_compile_options(AoCD8 PRIVATE /W4)
else()
    target_compile_options(AoCD8 PRIVATE -Wall -Wextra -pedantic)
endif()
