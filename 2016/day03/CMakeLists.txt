cmake_minimum_required(VERSION 4.3)
project(aoc2016d03 LANGUAGES CXX)

add_executable(aoc2016d03)

target_sources(aoc2016d03 PRIVATE
  src/main.cc
)

target_compile_features(aoc2016d03 PRIVATE cxx_std_23)

target_compile_options(aoc2016d03 PRIVATE
  $<$<CXX_COMPILER_ID:MSVC>:/W4>
  $<$<CXX_COMPILER_ID:GNU,Clang,AppleClang>:-Wall -Wextra -pedantic>
)
