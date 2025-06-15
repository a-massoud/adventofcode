#include <expected>
#include <fstream>
#include <iostream>
#include <print>

enum class Error { FileReadError };

std::expected<std::string, Error> read_input(const char *const fname);
std::string step(const std::string &prev);
std::string step_n(const std::string &initial, int steps);

int main(int argc, char *argv[]) {
  if (argc < 2) {
    std::println(std::cerr, "usage: {} <input file>", argv[0]);
    return 0;
  }

  auto input_res = read_input(argv[1]);
  if (!input_res) {
    std::println(std::cerr, "could not read from {}", argv[1]);
    return 1;
  }
  std::string input = *std::move(input_res);

  std::string part1 = step_n(input, 40);
  std::println("Part 1 result: {}", part1.length());

  return 0;
}

std::expected<std::string, Error> read_input(const char *const fname) {
  std::ifstream file(fname);
  if (!file.good()) {
    return std::unexpected(Error::FileReadError);
  }

  std::string line;
  std::getline(file, line);

  return line;
}

std::string step(const std::string &prev) {
  std::string next;
  next.reserve(prev.length());

  std::size_t current_run = 0;
  char current_char = 0;
  for (const char &cmp : prev) {
    if (cmp == current_char) {
      current_run += 1;
    } else {
      if (current_run > 0) {
        next.append(std::format("{:d}{:c}", current_run, current_char));
      }
      current_run = 1;
      current_char = cmp;
    }
  }
  if (current_run > 0) {
    next.append(std::format("{:d}{:c}", current_run, current_char));
  }

  return next;
}

std::string step_n(const std::string &initial, int steps) {
  std::string current = initial;

  for (int i = 0; i < steps; ++i) {
    current = step(current);
  }

  return current;
}
