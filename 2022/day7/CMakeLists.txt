cmake_minimum_required(VERSION 3.20)
project(AoCD7)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(AoCD7 src/main.cc)
target_include_directories(AoCD7 PRIVATE src)
if(MSVC)
    target_compile_options(AoCD7 PRIVATE /W4)
else()
    target_compile_options(AoCD7 PRIVATE -Wall -Wextra -pedantic)
endif()
