add_executable(aoc2015d10 main.cc)

target_compile_features(aoc2015d10 PRIVATE cxx_std_23)

if(MSVC)
  target_compile_options(aoc2015d10 PRIVATE /W4)
else()
  target_compile_options(aoc2015d10 PRIVATE -Wall -Wextra -pedantic)
endif()
