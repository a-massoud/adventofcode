#include <fstream>
#include <iostream>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <tuple>
#include <vector>

// This is... awful. Just awful. It's bad.
// It works though, and it's 10pm, and I'm not doing any better now!

const std::vector<std::string> readInputFile(const std::string &fname);
void parseStackLine(std::vector<std::vector<char>> &stacks,
                    const std::string &line);
const std::string
part1Answer(std::vector<std::vector<char>> stacks,
            const std::vector<std::tuple<int, int, int>> &instructions);
const std::string
part2Answer(std::vector<std::vector<char>> stacks,
            const std::vector<std::tuple<int, int, int>> &instructions);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "What's the file?\n";
        return 1;
    }

    const std::vector<std::string> input = readInputFile(argv[1]);

    std::vector<std::vector<char>> stacks;
    std::vector<std::tuple<int, int, int>> instructions;

    auto lineIt = input.begin();
    while (*lineIt != "")
        ++lineIt;
    auto instructionsLine = lineIt + 1;
    lineIt -= 2;
    for (std::size_t i = 0; i < lineIt->length(); i += 4)
        stacks.push_back({});
    while (lineIt != input.begin()) {
        parseStackLine(stacks, *lineIt);
        --lineIt;
    }
    parseStackLine(stacks, *lineIt);

    auto instructionsRegex =
        std::regex("move ([0-9]+) from ([0-9]+) to ([0-9]+)");
    std::smatch instructionsMatchResults;
    while (instructionsLine != input.end()) {
        std::regex_match(*instructionsLine, instructionsMatchResults,
                         instructionsRegex);
        instructions.push_back(
            {std::stoi(instructionsMatchResults[1].str()),
             std::stoi(instructionsMatchResults[2].str()) - 1,
             std::stoi(instructionsMatchResults[3].str()) - 1});
        ++instructionsLine;
    }

    std::cout << "CrateMover 9000: " << part1Answer(stacks, instructions)
              << '\n';
    std::cout << "CrateMover 9001: " << part2Answer(stacks, instructions)
              << '\n';

    return 0;
}

const std::vector<std::string> readInputFile(const std::string &fname) {
    std::vector<std::string> lines;

    std::ifstream file(fname);
    if (!file.is_open())
        throw std::runtime_error((std::stringstream()
                                  << "Failed to open " << fname
                                  << " for reading")
                                     .str());

    std::string line;
    while (std::getline(file, line))
        lines.push_back(line);

    return lines;
}

void parseStackLine(std::vector<std::vector<char>> &stacks,
                    const std::string &line) {
    for (std::size_t i = 0; i < line.length(); i += 4) {
        if (line[i + 1] != ' ')
            stacks[i / 4].push_back(line[i + 1]);
    }
}

const std::string
part1Answer(std::vector<std::vector<char>> stacks,
            const std::vector<std::tuple<int, int, int>> &instructions) {
    std::string r = "";

    for (auto instruction : instructions) {
        int num, from, to;
        std::tie(num, from, to) = instruction;

        for (int i = 0; i < num; ++i) {
            stacks[to].push_back(stacks[from].back());
            stacks[from].pop_back();
        }
    }

    for (auto stack : stacks)
        r += stack.back();

    return r;
}

const std::string
part2Answer(std::vector<std::vector<char>> stacks,
            const std::vector<std::tuple<int, int, int>> &instructions) {
    std::string r = "";

    for (auto instruction : instructions) {
        int num, from, to;
        std::tie(num, from, to) = instruction;

        std::vector<char> tmp;
        for (int i = 0; i < num; ++i) {
            tmp.push_back(stacks[from].back());
            stacks[from].pop_back();
        }

        for (int i = 0; i < num; ++i) {
            stacks[to].push_back(tmp.back());
            tmp.pop_back();
        }
    }

    for (auto stack : stacks)
        r += stack.back();

    return r;
}
