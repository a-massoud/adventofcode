#include <fstream>
#include <iostream>
#include <sstream>
#include <utility>
#include <vector>

struct ElfSection {
    int begin;
    int end;

    ElfSection() = default;
    ElfSection(int begin, int end) : begin(begin), end(end) {}
};

int countEngulfments(
    const std::vector<std::pair<ElfSection, ElfSection>> &elfPairs);
int countOverlaps(
    const std::vector<std::pair<ElfSection, ElfSection>> &elfPairs);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "C'mon, seriously?\n";
        return 1;
    }

    std::ifstream input(argv[1]);
    if (!input.is_open()) {
        std::cerr << "Failed to open " << argv[1] << " for reading.";
        return 1;
    }

    std::vector<std::pair<ElfSection, ElfSection>> elves;
    for (std::string line;
         std::getline(input, line) && !(line == "" && input.eof());) {
        std::stringstream ssline(line);
        std::pair<ElfSection, ElfSection> elfPair;
        char ch;
        ssline >> elfPair.first.begin >> ch >> elfPair.first.end >> ch >>
            elfPair.second.begin >> ch >> elfPair.second.end;
        if (ssline.fail()) {
            std::cerr << "Input line: \"" << line << "\" is broken.";
            return 1;
        }
        elves.push_back(elfPair);
    }

    int part1Engulfments = countEngulfments(elves);
    std::cout << "Engulfment count: " << part1Engulfments << "\n";

    int part2Overlaps = countOverlaps(elves);
    std::cout << "Overlap count: " << part2Overlaps << "\n";

    return 0;
}

int countEngulfments(
    const std::vector<std::pair<ElfSection, ElfSection>> &elfPairs) {
    int engulfments = 0;
    for (const auto &elfPair : elfPairs) {
        if ((elfPair.first.begin <= elfPair.second.begin &&
             elfPair.first.end >= elfPair.second.end) ||
            (elfPair.second.begin <= elfPair.first.begin &&
             elfPair.second.end >= elfPair.first.end)) {
            ++engulfments;
            continue;
        }
    }

    return engulfments;
}

int countOverlaps(
    const std::vector<std::pair<ElfSection, ElfSection>> &elfPairs) {
    int overlaps = 0;
    for (const auto &elfPair : elfPairs) {
        if ((elfPair.first.begin <= elfPair.second.begin &&
             elfPair.first.end >= elfPair.second.begin) ||
            (elfPair.second.begin <= elfPair.first.begin &&
             elfPair.second.end >= elfPair.first.begin)) {
            ++overlaps;
            continue;
        }
    }

    return overlaps;
}
