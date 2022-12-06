cmake_minimum_required(VERSION 3.20)
project(AoCD6)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(AoCD6 src/main.cc)
target_include_directories(AoCD6 PRIVATE src)
if(MSVC)
    target_compile_options(AoCD6 PRIVATE /W4)
else()
    target_compile_options(AoCD6 PRIVATE -Wall -Wextra -pedantic)
endif()
