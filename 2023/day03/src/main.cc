#include <cctype>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

// I had trouble remembering what coordinate was what, but also I wrote this
// before I had coffee, so who knows how this goes. It works though and that's
// what matters.
//
// I probably could have added some kind of check for part 2 to just stop at
// more than 2 numbers, but really some substring operations and `std::stoi`s
// are not that intensive and it would be really ugly

std::vector<std::string> read_input(const std::string &input_file_name);
bool is_part_num(const std::vector<std::string> &input, long line, long start,
                 long end);
long sum_part_nums(const std::vector<std::string> &input);
long sum_gear_nums(const std::vector<std::string> &input);
long get_gear_ratio(const std::vector<std::string> &input, long x, long y);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>\n";
        return 1;
    }

    std::vector<std::string> input = read_input(argv[1]);

    std::cout << "Part 1 result: " << sum_part_nums(input) << "\n";
    std::cout << "Part 2 result: " << sum_gear_nums(input) << "\n";

    return 0;
}

std::vector<std::string> read_input(const std::string &input_file_name) {
    std::vector<std::string> final_input;

    std::ifstream input_file(input_file_name);
    if (!input_file.good())
        return final_input;

    std::string line;
    while (std::getline(input_file, line))
        final_input.push_back(line);

    return final_input;
}

bool is_part_num(const std::vector<std::string> &input, long line, long start,
                 long end) {
    if (line >= static_cast<long>(input.size()))
        return false;
    if (start > end || start < 0 ||
        end > static_cast<long>(input[line].length()))
        return false;

    // first row
    if (line >= 1) {
        for (long x = start - 1; x <= end + 1; ++x) {
            if (x < 0 || x >= static_cast<long>(input[line - 1].length()))
                continue;
            if (input[line - 1][x] != '.' && !std::isdigit(input[line - 1][x]))
                return true;
        }
    }

    // ends
    if (start - 1 >= 0 && input[line][start - 1] != '.' &&
        !std::isdigit(input[line][start - 1])) {
        return true;
    }
    if (end + 1 < static_cast<long>(input[line].length()) &&
        input[line][end + 1] != '.' && !std::isdigit(input[line][end + 1])) {
        return true;
    }

    // last row
    if (line < static_cast<long>(input.size()) - 1) {
        for (long x = start - 1; x <= end + 1; ++x) {
            if (x < 0 || x >= static_cast<long>(input[line + 1].length()))
                continue;
            if (input[line + 1][x] != '.' && !std::isdigit(input[line + 1][x]))
                return true;
        }
    }

    return false;
}

long sum_part_nums(const std::vector<std::string> &input) {
    long total = 0;

    for (long i = 0; i < static_cast<long>(input.size()); ++i) {
        for (long j = 0; j < static_cast<long>(input[i].size()); ++j) {
            if (std::isdigit(input[i][j])) {
                long k = j;
                // move j to end of number
                for (; j < static_cast<long>(input[i].size()) &&
                       std::isdigit(input[i][j]);
                     ++j)
                    ;
                if (is_part_num(input, i, k, j - 1)) {
                    total += std::stoi(input[i].substr(k, j - k));
                }
            }
        }
    }

    return total;
}

long sum_gear_nums(const std::vector<std::string> &input) {
    long total = 0;

    for (long i = 0; i < static_cast<long>(input.size()); ++i) {
        for (long j = 0; j < static_cast<long>(input[i].size()); ++j) {
            if (input[i][j] != '*')
                continue;
            long gear_ratio = get_gear_ratio(input, j, i);
            total += gear_ratio;
        }
    }

    return total;
}

long get_gear_ratio(const std::vector<std::string> &input, long x, long y) {
    std::vector<long> numbers;

    // first row
    if (y >= 1) {
        for (long i = x - 1; i <= x + 1; ++i) {
            if (i < 0 || i >= static_cast<long>(input[y - 1].length()) ||
                !std::isdigit(input[y - 1][i]))
                continue;

            // find beginning of number
            long j = i;
            while (j >= 0 && std::isdigit(input[y - 1][j]))
                --j;
            ++j;

            // advance i to end of number
            while (i < static_cast<long>(input[y - 1].length()) &&
                   std::isdigit(input[y - 1][i]))
                ++i;
            --i;

            numbers.push_back(std::stol(input[y - 1].substr(j, i - j + 1)));
        }
    }

    // edges
    if (x >= 1 && std::isdigit(input[y][x - 1])) {
        // find beginning of number
        long j = x - 1;
        while (j >= 0 && std::isdigit(input[y][j]))
            --j;
        ++j;

        numbers.push_back(std::stol(input[y].substr(j, x - j)));
    }
    if (x < static_cast<long>(input[y].length()) - 1 &&
        std::isdigit(input[y][x + 1])) {
        // find beginning of number
        long j = x + 1;
        while (j < static_cast<long>(input[y].length()) - 1 &&
               std::isdigit(input[y][j]))
            ++j;
        --j;

        numbers.push_back(std::stol(input[y].substr(x + 1, j - x)));
    }

    // last row
    if (y < static_cast<long>(input.size()) - 1) {
        for (long i = x - 1; i <= x + 1; ++i) {
            if (i < 0 || i >= static_cast<long>(input[y + 1].length()) ||
                !std::isdigit(input[y + 1][i]))
                continue;

            // find beginning of number
            long j = i;
            while (j >= 0 && std::isdigit(input[y + 1][j]))
                --j;
            ++j;

            // advance i to end of number
            while (i < static_cast<long>(input[y + 1].length()) &&
                   std::isdigit(input[y + 1][i]))
                ++i;
            --i;

            numbers.push_back(std::stol(input[y + 1].substr(j, i - j + 1)));
        }
    }

    // not a gear
    if (numbers.size() != 2)
        return 0;

    return numbers[0] * numbers[1];
}
