#include <fstream>
#include <iostream>
#include <ranges>
#include <regex>
#include <string>
#include <string_view>
#include <vector>

// This took way longer than it should have. I didn't realize the overlapping
// problem until too late, and not wanting to stare at regex for any longer than
// necessary I just hacked together nonsense that worked. It's very ugly, and
// there's probably a better way to do it, but it works and I hate regex..

std::vector<std::string> read_input(const std::string &file_name);
long part1(const std::vector<std::string> &input);
int name_to_value(const std::string &name);
long part2(const std::vector<std::string> &input);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>\n";
        return 1;
    }

    const std::vector<std::string> input = read_input(argv[1]);

    std::cout << "Part 1 result: " << part1(input) << "\n";
    std::cout << "Part 2 result: " << part2(input) << "\n";

    return 0;
}

std::vector<std::string> read_input(const std::string &file_name) {
    std::vector<std::string> lines;

    std::ifstream input_file(file_name);
    if (!input_file.good())
        return lines;

    std::string line;
    while (!input_file.eof() && std::getline(input_file, line))
        lines.push_back(line);

    return lines;
}

long part1(const std::vector<std::string> &input) {
    long result = 0;

    for (const auto &line : input) {
        int line_value = 0;
        // first digit
        for (const char &ch : line) {
            if ('0' <= ch && ch <= '9') {
                line_value += 10 * (ch - '0');
                break;
            }
        }

        // second digit
        for (const char &ch : std::ranges::reverse_view(line)) {
            if ('0' <= ch && ch <= '9') {
                line_value += (ch - '0');
                break;
            }
        }

        result += line_value;
    }

    return result;
}

int name_to_value(const std::string &name) {
    if (name.length() == 1 && '0' <= name[0] && name[0] <= '9')
        return name[0] - '0';

    if (name == "zero")
        return 0;
    else if (name == "one")
        return 1;
    else if (name == "two")
        return 2;
    else if (name == "three")
        return 3;
    else if (name == "four")
        return 4;
    else if (name == "five")
        return 5;
    else if (name == "six")
        return 6;
    else if (name == "seven")
        return 7;
    else if (name == "eight")
        return 8;
    else if (name == "nine")
        return 9;

    return 0;
}

long part2(const std::vector<std::string> &input) {
    long result = 0;

    std::regex number_regex("[0-9]|zero|one|two|three|four|five|six|seven|eight|nine");
    std::regex rev_number_regex("[0-9]|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin");

    for (const std::string &line : input) {
        // first digit
        std::smatch number_match;
        std::regex_search(line, number_match, number_regex);
        result += (name_to_value(number_match.str()) * 10);

        // second digit
        std::smatch rev_number_match;
        std::string rev_line = line;
        std::reverse(rev_line.begin(), rev_line.end());
        std::regex_search(rev_line, rev_number_match, rev_number_regex);
        std::string rev_number = rev_number_match.str();
        std::reverse(rev_number.begin(), rev_number.end());
        result += name_to_value(rev_number);
    }

    return result;
}
