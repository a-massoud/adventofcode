#include "dijkstra.h"
#include "util.h"
#include <fstream>
#include <iostream>
#include <tuple>
#include <vector>

// today was nice and simple, just basic DSA.
// well hell, actually, not even DS, the STL did that for me.
// well it was nice having a problem and immediately knowing what to do, and
// actually implementing dijkstra off the top of my head was good practice.

// Parse the input
//   Return: (heightmap, starting location, ending location)
std::tuple<std::vector<std::vector<char>>, Point, Point>
parseInput(const std::string &fname);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "NO INPUT GIVEN, PLEASE GIVE ME SOME\n";
        return 1;
    }

    Point start, end;
    std::vector<std::vector<char>> map;
    std::tie(map, start, end) = parseInput(argv[1]);
    if (start.x == -1 || end.x == -1) {
        std::cerr << "Failed to parse input.\n";
        return 1;
    }

    std::cout << "Start: (" << start.x << ", " << start.y << ")\nEnd: ("
              << end.x << ", " << end.y << ")\n";
    for (const auto &line : map) {
        for (const auto &ch : line)
            std::cout << static_cast<char>(ch + 'a');
        std::cout << '\n';
    }

    std::vector<Point> part1 = runDijkstra(
        map, start, [&end](const Point &p) { return p == end; }, true);
    std::cout << "Part 1 solution:\n  Path:\n    ";
    for (const auto &p : part1) {
        std::cout << "(" << p.x << ", " << p.y << "), ";
    }
    // path includes origin
    std::cout << "\n  Steps: " << part1.size() - 1 << "\n";

    std::vector<Point> part2 = runDijkstra(
        map, end, [&map](const Point &p) { return map[p.y][p.x] == 'a' - 'a'; },
        false);
    std::cout << "\nPart 2 solution:\n  Path:\n    ";
    for (const auto &p : part2) {
        std::cout << "(" << p.x << ", " << p.y << "), ";
    }
    // path includes origin
    std::cout << "\n  Steps: " << part2.size() - 1 << "\n";

    return 0;
}

std::tuple<std::vector<std::vector<char>>, Point, Point>
parseInput(const std::string &fname) {
    Point start = {-1, -1};
    Point end = {-1, -1};
    std::vector<std::vector<char>> map;

    std::ifstream input(fname);
    if (!input)
        return std::make_tuple(map, start, end);

    std::string line;
    while (std::getline(input, line)) {
        map.push_back(std::vector<char>());
        for (const auto &ch : line) {
            if (ch == 'S') {
                map.back().push_back('a' - 'a');
                start = {static_cast<int>((map.back().end() - 1) -
                                          map.back().begin()),
                         static_cast<int>((map.end() - 1) - map.begin())};
            } else if (ch == 'E') {
                map.back().push_back('z' - 'a');
                end = {static_cast<int>((map.back().end() - 1) -
                                        map.back().begin()),
                       static_cast<int>((map.end() - 1) - map.begin())};
            } else {
                map.back().push_back(ch - 'a');
            }
        }
    }

    return std::make_tuple(map, start, end);
}
