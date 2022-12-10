cmake_minimum_required(VERSION 3.20)
project(AoCD1)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(AoCD1 src/main.cc)
target_include_directories(AoCD1 PRIVATE src)
if(MSVC)
    target_compile_options(AoCD1 PRIVATE /W4)
else()
    target_compile_options(AoCD1 PRIVATE -Wall -Wextra -pedantic)
endif()
