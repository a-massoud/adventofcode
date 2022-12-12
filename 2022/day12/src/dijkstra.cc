#include "dijkstra.h"
#include "util.h"
#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <limits>
#include <queue>
#include <vector>

class DijkstraNode {
  private:
    std::vector<Point> neighbors_;
    Point location_;
    long dist_;
    DijkstraNode *parent_;

  public:
    DijkstraNode(Point location, const std::vector<Point> &neighbors);

    const std::vector<Point> &neighbors() { return neighbors_; }
    const Point &location() const { return location_; }
    long &distance() { return dist_; }
    const long &distance() const { return dist_; }
    DijkstraNode *&parent() { return parent_; }
    const DijkstraNode *const &parent() const { return parent_; }
};

std::vector<std::vector<DijkstraNode>>
constructNodes(const std::vector<std::vector<char>> heightMap, bool dir);

std::vector<Point> runDijkstra(const std::vector<std::vector<char>> &heightMap,
                               const Point start,
                               std::function<bool(const Point &)> endCondition,
                               bool dir) {
    std::vector<Point> path;

    std::vector<std::vector<DijkstraNode>> nodes =
        constructNodes(heightMap, dir);
    nodes[start.y][start.x].distance() = 0;

    std::priority_queue<DijkstraNode *, std::vector<DijkstraNode *>,
                        decltype([](DijkstraNode *a, DijkstraNode *b) -> bool {
                            return a->distance() > b->distance();
                        })>
        queue;
    queue.push(&(nodes[start.y][start.x]));

    while (!queue.empty()) {
        DijkstraNode *node = queue.top();
        queue.pop();

        if (endCondition(node->location())) {
            const DijkstraNode *tNode = node;
            while (tNode) {
                path.push_back(tNode->location());
                tNode = tNode->parent();
            }
            if (dir)
                std::reverse(path.begin(), path.end());
            break;
        }

        for (auto &neighbor : node->neighbors()) {
            if (nodes[neighbor.y][neighbor.x].distance() >
                node->distance() + 1) {
                nodes[neighbor.y][neighbor.x].distance() = node->distance() + 1;
                nodes[neighbor.y][neighbor.x].parent() = node;
                queue.push(&nodes[neighbor.y][neighbor.x]);
            }
        }
    }

    return path;
}

DijkstraNode::DijkstraNode(Point location, const std::vector<Point> &neighbors)
    : neighbors_(neighbors), location_(location) {
    dist_ = std::numeric_limits<long>::max();
    parent_ = nullptr;
}

std::vector<std::vector<DijkstraNode>>
constructNodes(const std::vector<std::vector<char>> heightMap, bool dir) {
    std::vector<std::vector<DijkstraNode>> nodes;
    int mulFactor = dir ? 1 : -1;

    for (std::size_t i = 0; i < heightMap.size(); ++i) {
        if (nodes.size() == i)
            nodes.push_back(std::vector<DijkstraNode>());
        for (std::size_t j = 0; j < heightMap[i].size(); ++j) {
            std::vector<Point> neighbors;

            // up
            if (i >= 1 &&
                (heightMap[i - 1][j] - heightMap[i][j]) * mulFactor <= 1)
                neighbors.push_back(
                    {static_cast<int>(j), static_cast<int>(i - 1)});

            // down
            if (i <= heightMap.size() - 2 &&
                (heightMap[i + 1][j] - heightMap[i][j]) * mulFactor <= 1)
                neighbors.push_back(
                    {static_cast<int>(j), static_cast<int>(i + 1)});

            // left
            if (j >= 1 &&
                (heightMap[i][j - 1] - heightMap[i][j]) * mulFactor <= 1)
                neighbors.push_back(
                    {static_cast<int>(j - 1), static_cast<int>(i)});

            // right
            if (j <= heightMap[i].size() - 2 &&
                (heightMap[i][j + 1] - heightMap[i][j]) * mulFactor <= 1)
                neighbors.push_back(
                    {static_cast<int>(j + 1), static_cast<int>(i)});

            nodes[i].push_back(DijkstraNode(
                {static_cast<int>(j), static_cast<int>(i)}, neighbors));
        }
    }

    return nodes;
}
