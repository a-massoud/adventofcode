#include <algorithm>
#include <cctype>
#include <charconv>
#include <cstddef>
#include <cstdint>
#include <expected>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <print>
#include <span>
#include <string>
#include <system_error>
#include <variant>
#include <vector>

namespace {

struct Triangle {
  std::int64_t a;
  std::int64_t b;
  std::int64_t c;
};

namespace read_input_error {
struct FileOpen {};

struct BadLine {
  std::string line;
};

struct ReadLine {};
}; // namespace read_input_error

using ReadInputError =
    std::variant<read_input_error::FileOpen, read_input_error::BadLine,
                 read_input_error::ReadLine>;

auto readInput(const std::filesystem::path &inputFile)
    -> std::expected<std::vector<std::int64_t>, ReadInputError> {
  std::ifstream input{inputFile};

  if (!input) {
    return std::unexpected{read_input_error::FileOpen{}};
  }

  std::vector<std::int64_t> values;

  std::string line;
  while (std::getline(input, line)) {
    std::array<std::int64_t, 3> row{};
    auto p{line.cbegin()};

    for (std::size_t i{0}; i < row.size(); ++i) {
      while (p != line.cend() &&
             (std::isspace(static_cast<unsigned char>(*p)) != 0)) {
        ++p;
      }
      if (p == line.cend()) {
        return std::unexpected{
            read_input_error::BadLine{.line = std::move(line)}};
      }

      std::int64_t value{};
      const auto start{p};
      auto [ptr, ec] = std::from_chars(std::to_address(p),
                                       std::to_address(line.cend()), value);
      if (ec != std::errc{} || ptr == std::to_address(start)) {
        return std::unexpected{
            read_input_error::BadLine{.line = std::move(line)}};
      }

      row.at(i) = value;
      p += ptr - std::to_address(start);
    }
    while (p != line.cend() &&
           (std::isspace(static_cast<unsigned char>(*p)) != 0)) {
      ++p;
    }
    if (p != line.cend()) {
      return std::unexpected{
          read_input_error::BadLine{.line = std::move(line)}};
    }
    values.append_range(row);
  }

  if (input.bad()) {
    return std::unexpected{read_input_error::ReadLine{}};
  }

  return values;
}

enum class ExtractionError : std::uint8_t { IncorrectNumber };

auto extractRowTriangles(std::span<const std::int64_t> input)
    -> std::expected<std::vector<Triangle>, ExtractionError> {
  if (input.size() % 3 != 0) {
    return std::unexpected{ExtractionError::IncorrectNumber};
  }

  std::vector<Triangle> triangles;
  for (std::size_t i{0}; i + 2 < input.size(); i += 3) {
    std::array<std::int64_t, 3> triangle{input[i], input[i + 1], input[i + 2]};
    std::ranges::sort(triangle);
    triangles.push_back(
        Triangle{.a = triangle[0], .b = triangle[1], .c = triangle[2]});
  }
  return triangles;
}

auto extractColTriangles(std::span<const std::int64_t> input)
    -> std::expected<std::vector<Triangle>, ExtractionError> {
  if (input.size() % 9 != 0) {
    return std::unexpected{ExtractionError::IncorrectNumber};
  }

  std::vector<Triangle> triangles;
  for (std::size_t j{0}; j + 8 < input.size(); j += 9) {
    for (std::size_t i{j}; i < j + 3; i++) {
      std::array<std::int64_t, 3> triangle{input[i], input[i + 3],
                                           input[i + 6]};
      std::ranges::sort(triangle);
      triangles.push_back(
          Triangle{.a = triangle[0], .b = triangle[1], .c = triangle[2]});
    }
  }
  return triangles;
}

auto getNPossible(std::span<const Triangle> triangles) -> std::size_t {
  return std::ranges::count_if(
      triangles, [](const auto &v) -> auto { return v.c < v.a + v.b; });
}

template <class... Ts> struct overloaded : Ts... {
  using Ts::operator()...;
};

}; // namespace

auto main(int argc, char *argv[]) -> int {
  const std::span args{argv, static_cast<std::size_t>(argc)};

  if (args.size() < 2) {
    std::println(std::cerr, "no argument provided");
    return 1;
  }

  const auto input{readInput(args[1])};
  if (!input) {
    std::visit(overloaded{[](const read_input_error::FileOpen &) -> void {
                            std::println(std::cerr, "failed to open file");
                          },
                          [](const read_input_error::BadLine &e) -> void {
                            std::println(std::cerr, "bad line: `{}`", e.line);
                          },
                          [](const read_input_error::ReadLine &) -> void {
                            std::println(std::cerr, "failed to read line");
                          }},
               input.error());
    return 1;
  }

  const auto part1Triangles{extractRowTriangles(*input)};
  if (!part1Triangles) {
    switch (part1Triangles.error()) {
    case ExtractionError::IncorrectNumber:
      std::println(std::cerr, "incorrect number of elements to form triangles");
      return 1;
    }
  }
  const auto part1{getNPossible(*part1Triangles)};
  std::println("Part 1: {}", part1);

  const auto part2Triangles{extractColTriangles(*input)};
  if (!part2Triangles) {
    switch (part2Triangles.error()) {
    case ExtractionError::IncorrectNumber:
      std::println(std::cerr, "incorrect number of elements to form triangles");
      return 1;
    }
  }
  const auto part2{getNPossible(*part2Triangles)};
  std::println("Part 2: {}", part2);

  return 0;
}
