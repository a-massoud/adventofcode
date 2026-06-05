// Should I have been grading? Yes. Did I do this instead? Also yes.

#include <algorithm>
#include <array>
#include <cctype>
#include <cerrno>
#include <charconv>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <expected>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <print>
#include <ranges>
#include <span>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

namespace {

class Entry {
public:
  struct Params {
    std::string name;
    std::int64_t id{0};
    std::string checksum;
  };

  enum class Error : std::uint8_t { BadValue, BadChecksum };

  static auto create(Params p) -> std::expected<Entry, Error> {
    for (const auto &ch : p.name) {
      if (ch != '-' && (ch < 'a' || ch > 'z')) {
        return std::unexpected{Error::BadValue};
      }
    }

    if (p.checksum.size() != 5) {
      return std::unexpected{Error::BadChecksum};
    }
    for (const auto &ch : p.checksum) {
      if (ch < 'a' || ch > 'z') {
        return std::unexpected{Error::BadChecksum};
      }
    }

    return Entry{std::move(p.name), p.id, std::move(p.checksum)};
  }

  [[nodiscard]] auto isReal() const -> bool {
    std::array<std::pair<char, std::size_t>, 26> frequencies{};
    for (char ch{'a'}; ch <= 'z'; ++ch) {
      frequencies.at(ch - 'a') = {ch, 0};
    }

    for (const auto &ch : mName) {
      if (ch != '-') {
        ++frequencies.at(ch - 'a').second;
      }
    }

    std::ranges::sort(frequencies, [](auto a, auto b) -> bool {
      if (a.second == b.second) {
        return a.first < b.first;
      }
      return a.second > b.second;
    });

    return std::ranges::all_of(
        std::views::zip(mChecksum, frequencies),
        [](const auto &z) -> bool { return get<0>(z) == get<1>(z).first; });
  }

  [[nodiscard]] auto decrypt() const -> std::string {
    std::string name;
    name.reserve(mName.size());

    for (const auto &ch : mName) {
      if (ch == '-') {
        name.push_back('-');
      } else {
        const auto offset{
            (((static_cast<std::int64_t>(ch - 'a') + mId) % 26) + 26) % 26};
        name.push_back(static_cast<char>('a' + offset));
      }
    }

    return name;
  }

  [[nodiscard]] auto id() const -> std::int64_t { return mId; }

private:
  Entry(std::string &&name, std::int64_t id, std::string &&checksum)
      : mName{std::move(name)}, mId{id}, mChecksum{std::move(checksum)} {}

  std::string mName;
  std::int64_t mId{0};
  std::string mChecksum;
};

namespace read_input_error {

struct OpenFile {
  std::string msg;
};

struct ReadLine {
  std::string msg;
};

struct BadLine {
  std::size_t no;
  std::string line;
};

} // namespace read_input_error

using ReadInputError =
    std::variant<read_input_error::OpenFile, read_input_error::ReadLine,
                 read_input_error::BadLine>;

auto toString(const ReadInputError &e) -> std::string {
  return std::visit(
      [](const auto &e) -> std::string {
        using T = std::decay_t<decltype(e)>;

        if constexpr (std::is_same_v<T, read_input_error::OpenFile>) {
          return std::format("failed to open file: {}", e.msg);
        } else if constexpr (std::is_same_v<T, read_input_error::ReadLine>) {
          return std::format("failed to read line: {}", e.msg);
        } else if constexpr (std::is_same_v<T, read_input_error::BadLine>) {
          return std::format("bad line {} (`{}`)", e.no, e.line);
        } else {
          static_assert(!std::is_same_v<T, T>, "incomplete visit");
        }
      },
      e);
}

auto parseLine(const std::string_view line, std::size_t lineno)
    -> std::expected<Entry, read_input_error::BadLine> {
  std::size_t start{0};
  auto pos{start};
  while (pos < line.size() &&
         (line[pos] == '-' || ('a' <= line[pos] && line[pos] <= 'z'))) {
    ++pos;
  }
  if (pos >= line.size() || pos == start || line[pos - 1] != '-' ||
      std::isdigit(static_cast<unsigned char>(line[pos])) == 0) {
    return std::unexpected{
        read_input_error::BadLine{.no = lineno, .line = std::string{line}}};
  }

  std::string name{line.substr(start, pos - start - 1)};

  start = pos;
  while (pos < line.size() && line[pos] != '[') {
    ++pos;
  }
  if (pos >= line.size()) {
    return std::unexpected{
        read_input_error::BadLine{.no = lineno, .line = std::string{line}}};
  }

  std::int64_t id{0};
  auto [ptr, err] = std::from_chars(&line[start], &line[pos], id);
  if (err != std::errc{} || ptr != &line[pos]) {
    return std::unexpected{
        read_input_error::BadLine{.no = lineno, .line = std::string{line}}};
  }

  start = pos + 1;
  pos = start;
  while (pos < line.size() && line[pos] != ']') {
    ++pos;
  }
  if (pos != line.size() - 1) {
    return std::unexpected{
        read_input_error::BadLine{.no = lineno, .line = std::string{line}}};
  }

  std::string checksum{line.substr(start, pos - start)};

  auto ret{Entry::create(Entry::Params{
      .name = std::move(name), .id = id, .checksum = std::move(checksum)})};

  if (!ret) {
    return std::unexpected{
        read_input_error::BadLine{.no = lineno, .line = std::string{line}}};
  }

  return *ret;
}

auto readInput(const std::filesystem::path &path)
    -> std::expected<std::vector<Entry>, ReadInputError> {
  std::ifstream input{path};

  if (!input) {
    return std::unexpected{
        read_input_error::OpenFile{.msg = std::strerror(errno)}};
  }

  std::vector<Entry> ret;

  std::string line;
  std::size_t lineno{0};
  while (std::getline(input, line)) {
    ++lineno;

    auto entry{parseLine(line, lineno)};
    if (!entry) {
      return std::unexpected{entry.error()};
    }

    ret.push_back(*entry);
  }

  if (input.fail() && !input.eof()) {
    return std::unexpected{
        read_input_error::ReadLine{.msg = std::strerror(errno)}};
  }

  return ret;
}

auto sumEntries(const std::span<const Entry> entries) -> std::int64_t {
  std::int64_t sum{0};

  for (const auto &entry : entries) {
    sum += entry.id();
  }

  return sum;
}

} // namespace

auto main(int argc, char *argv[]) -> int {
  std::span args{argv, static_cast<std::size_t>(argc)};

  if (args.size() < 2) {
    std::println(std::cerr, "no input provided");
    return 1;
  }

  auto input{readInput(args[1])};
  if (!input) {
    std::println(std::cerr, "failed to read file: {}", toString(input.error()));
    return 1;
  }
  std::erase_if(*input, [](const Entry &e) -> bool { return !e.isReal(); });

  auto sum{sumEntries(*input)};
  std::println("Sum of valid IDs: {}", sum);

  bool found{false};
  for (const auto &entry : *input) {
    if (entry.decrypt() == "northpole-object-storage") {
      std::println("Northpole object storage ID: {}", entry.id());
      found = true;
      break;
    }
  }
  if (!found) {
    std::println("No room called northpole-object-storage found");
    return 1;
  }

  return 0;
}
