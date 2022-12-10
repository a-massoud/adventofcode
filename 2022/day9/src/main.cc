#include <cstdlib>
#include <array>
#include <fstream>
#include <iostream>
#include <unordered_set>
#include <utility>
#include <vector>

// omg look how much easier it is when we don't need to worry about implementing
// our own set data type. spent far too much on this just because i didn't
// realize i was parsing my input wrong ("R" doesn't mean up), but otherwise was
// pretty easy, and i think my solution is fairly efficient, not the best but
// fairly good.

enum class Dir { Right, Up, Left, Down };

std::vector<Dir> readInstructions(const std::string &fname);
std::pair<long, long> nextTailPos(std::pair<long, long> head,
                                  std::pair<long, long> tail);
long part1Results(const std::vector<Dir> &instructions);
long part2Results(const std::vector<Dir> &instructions);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "i need an argument\n";
        return 1;
    }

    std::vector<Dir> instructions = readInstructions(argv[1]);

    long part1 = part1Results(instructions);
    std::cout << "Part 1: " << part1 << "\n";

    long part2 = part2Results(instructions);
    std::cout << "Part 2: " << part2 << "\n";
}

std::vector<Dir> readInstructions(const std::string &fname) {
    std::vector<Dir> instructions;

    std::ifstream inputFile(fname);
    if (!inputFile)
        return instructions;

    std::string line;
    while (std::getline(inputFile, line)) {
        Dir dir = (line[0] == 'R')
                      ? Dir::Right
                      : ((line[0] == 'U')
                             ? Dir::Up
                             : ((line[0] == 'L') ? Dir::Left : Dir::Down));

        int count = std::stoi(line.substr(2));
        for (int i = 0; i < count; ++i)
            instructions.push_back(dir);
    }

    return instructions;
}

long part1Results(const std::vector<Dir> &instructions) {
    auto pointHash = []<typename T1, typename T2>(std::pair<T1, T2> point) {
        auto hash1 = std::hash<T1>{}(point.first);
        auto hash2 = std::hash<T2>{}(point.second);

        if (hash1 != hash2)
            return hash1 ^ hash2;
        return hash1;
    };
    std::unordered_set<std::pair<long, long>, decltype(pointHash)> visited;

    std::pair<long, long> head(0, 0);
    std::pair<long, long> tail(0, 0);
    visited.insert(tail);

    for (auto &instruction : instructions) {
        switch (instruction) {
        case Dir::Right:
            ++head.first;
            break;

        case Dir::Up:
            ++head.second;
            break;

        case Dir::Left:
            --head.first;
            break;

        case Dir::Down:
            --head.second;
            break;
        }

        tail = nextTailPos(head, tail);
        visited.insert(tail);

    }

    return visited.size();
}

std::pair<long, long> nextTailPos(std::pair<long, long> head,
                                  std::pair<long, long> tail) {
    int xDiff = head.first - tail.first;
    int yDiff = head.second - tail.second;

    // is touching
    if (-1 <= xDiff && xDiff <= 1 && -1 <= yDiff && yDiff <= 1) {
        return tail;
    }

    auto sgn = [](int val) {
        if (val < 0)
            return -1;
        if (val > 0)
            return 1;
        return 0;
    };

    int dx = sgn(xDiff);
    int dy = sgn(yDiff);

    tail.first += dx;
    tail.second += dy;

    return tail;
}

long part2Results(const std::vector<Dir> &instructions) {
    auto pointHash = []<typename T1, typename T2>(std::pair<T1, T2> point) {
        auto hash1 = std::hash<T1>{}(point.first);
        auto hash2 = std::hash<T2>{}(point.second);

        if (hash1 != hash2)
            return hash1 ^ hash2;
        return hash1;
    };
    std::unordered_set<std::pair<long, long>, decltype(pointHash)> visited;

    std::array<std::pair<long, long>, 10> rope;
    for (int i = 0; i < 10; ++i)
        rope[i] = {0, 0};

    for (auto &instruction : instructions) {
        switch (instruction) {
        case Dir::Right:
            ++rope[0].first;
            break;

        case Dir::Up:
            ++rope[0].second;
            break;

        case Dir::Left:
            --rope[0].first;
            break;

        case Dir::Down:
            --rope[0].second;
            break;
        }

        for (int i = 1; i < 10; ++i)
            rope[i] = nextTailPos(rope[i - 1], rope[i]);

        visited.insert(rope[9]);
    }

    return visited.size();
}
