cmake_minimum_required(VERSION 3.20)
project(aoc2015d4)

find_package(OpenSSL REQUIRED COMPONENTS Crypto)

add_executable(aoc2015d4 src/main.cc)
target_link_libraries(aoc2015d4 PRIVATE OpenSSL::Crypto OpenSSL::applink)

set_property(TARGET aoc2015d4 PROPERTY CXX_STANDARD 17)
set_property(TARGET aoc2015d4 PROPERTY CXX_STANDARD_REQUIRED ON)

if(MSVC)
    target_compile_options(aoc2015d4 PRIVATE /W4)
else()
    target_compile_options(aoc2015d4 PRIVATE -Wall -Wextra -pedantic)
endif()
