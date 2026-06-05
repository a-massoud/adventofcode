cmake_minimum_required(VERSION 4.3)
project(aoc2016d05 LANGUAGES CXX)

find_package(OpenSSL REQUIRED)

add_executable(aoc2016d05)

target_sources(aoc2016d05 PRIVATE
  src/main.cc
)

target_link_libraries(aoc2016d05 PRIVATE
  OpenSSL::SSL
)

target_compile_features(aoc2016d05 PRIVATE cxx_std_23)

target_compile_options(aoc2016d05 PRIVATE
  $<$<CXX_COMPILER_ID:MSVC>:/W4>
  $<$<CXX_COMPILER_ID:GNU,Clang,AppleClang>:-Wall -Wextra -pedantic>
)
