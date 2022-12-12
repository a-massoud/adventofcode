#ifndef ADVENTOFCODE_2022_DAY12_DIJKSTRA_H_YT401MW8
#define ADVENTOFCODE_2022_DAY12_DIJKSTRA_H_YT401MW8

#include "util.h"
#include <functional>
#include <vector>

std::vector<Point> runDijkstra(const std::vector<std::vector<char>> &heightMap,
                               const Point start,
                               std::function<bool(const Point &)> endCondition,
                               bool dir);

#endif
