#include <cstddef>
#include <exception>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

std::vector<std::string> read_lines(const std::string &file_name);
std::string parse_line(const std::string &raw_line);
std::string sanitize_line(const std::string &raw_line);
int run_part1(const std::vector<std::string> &lines);
int run_part2(const std::vector<std::string> &lines);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>\n";
        return 0;
    }

    std::vector<std::string> lines = read_lines(argv[1]);

    std::cout << "Part 1 result: " << run_part1(lines) << "\n";
    std::cout << "Part 2 result: " << run_part2(lines) << "\n";

    return 0;
}

std::vector<std::string> read_lines(const std::string &file_name) {
    // copy whole file into memory
    std::stringstream source;
    {
        std::ifstream file(file_name);
        if (!file.good())
            throw std::invalid_argument("failed to open file `" + file_name +
                                        "` for reading");
        source << file.rdbuf();
    }

    std::vector<std::string> lines;
    std::string line;
    while (std::getline(source, line))
        lines.push_back(line);

    return lines;
}

std::string parse_line(const std::string &raw_line) {
    if (raw_line.length() < 2 || raw_line.front() != '"' ||
        raw_line.back() != '"')
        throw std::invalid_argument("line `" + raw_line +
                                    "` is not formatted correctly");

    std::string line;
    for (std::size_t i = 1; i < raw_line.length() - 1; ++i) {
        if (raw_line[i] != '\\') {
            line.push_back(raw_line[i]);
            continue;
        }

        ++i;
        switch (raw_line[i]) {
        case '\\':
        case '"':
            line.push_back(raw_line[i]);
            break;

        case 'x':
            if (i > raw_line.length() - 3)
                throw std::invalid_argument("line `" + raw_line +
                                            "` contains \\x too close to end");
            line.push_back(static_cast<char>(
                std::stoi(raw_line.substr(i + 1, i + 3), nullptr, 16)));
            i += 2;
            break;
        }
    }
    return line;
}

std::string sanitize_line(const std::string &raw_line) {
    std::string line = "\"";

    for (const auto &ch : raw_line) {
        switch (ch) {
        case '\\':
            line.append("\\\\");
            break;
        case '"':
            line.append("\\\"");
            break;
        default:
            line.push_back(ch);
            break;
        }
    }
    line.push_back('\"');

    return line;
}

int run_part1(const std::vector<std::string> &lines) {
    int total = 0;

    for (const auto &line : lines) {
        std::string clean_line = parse_line(line);
        total += line.size() - clean_line.size();
    }

    return total;
}

int run_part2(const std::vector<std::string> &lines) {
    int total = 0;

    for (const auto &line : lines) {
        std::string clean_line = sanitize_line(line);
        total += clean_line.size() - line.size();
    }

    return total;
}
