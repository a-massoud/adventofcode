#include <chrono>
#include <cmath>
#include <fstream>
#include <iostream>
#include <limits>
#include <unordered_set>

// I regret to say that this is the day where I looked things up.
// I just had no idea of how to go about optimizing part 2, I kept on thinking
// along the lines of constructing some polygon and *then* testing all the
// lines. Damn. Made it 15 days though.

struct Point {
    long x;
    long y;

    Point() : x(0), y(0) {}
    Point(long x, long y) : x(x), y(y) {}

    friend bool operator==(const Point &p1, const Point &p2);
};

class Sensor {
  private:
    Point pos_;
    long range_;

  public:
    Sensor(Point pos, long range);
    Sensor(Point pos, Point beaconPos);

    const Point &pos() const { return pos_; }
    long range() const { return range_; }
    bool isInRange(const Point &pos) const;

    friend bool operator==(const Sensor &s1, const Sensor &s2);
};

namespace std {
template <> struct hash<Sensor> {
    size_t operator()(const Sensor &s) const {
        auto x = hash<long>{}(s.pos().x);
        x ^= hash<long>{}(s.pos().y);
        x ^= hash<long>{}(s.range());
        return x;
    }
};

template <> struct hash<Point> {
    size_t operator()(const Point &p) const {
        auto x = hash<long>{}(p.x);
        x ^= hash<long>{}(p.y);
        return x;
    }
};
} // namespace std

class SensorSet {
  private:
    std::unordered_set<Sensor> set_;
    Point topLeft_;
    Point bottomRight_;

  public:
    SensorSet();

    const auto &set() const { return set_; }

    void push(const Sensor &sensor);
    bool contains(const Sensor &sensor) const;
    bool canBeBeacon(const Point &p) const;
    std::pair<std::pair<long, long>, std::pair<long, long>> bounds() const;
};

long manhattanDistance(Point p1, Point p2);

std::pair<SensorSet, std::unordered_set<Point>>
parseInput(const std::string &fname);
long part1Results(const SensorSet &sensors,
                  const std::unordered_set<Point> &beacons);
long part2Results(const SensorSet &sensors);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Enter some input\n";
        return 1;
    }

    SensorSet sensors;
    std::unordered_set<Point> beacons;
    std::tie(sensors, beacons) = parseInput(argv[1]);

    auto cTime = std::chrono::system_clock::now();
    auto part1 = part1Results(sensors, beacons);
    std::cout << "Part 1 results: " << part1 << "\nTook "
              << static_cast<double>(
                     std::chrono::duration_cast<std::chrono::milliseconds>(
                         std::chrono::system_clock::now() - cTime)
                         .count()) /
                     1000.0
              << " seconds.\n";

    cTime = std::chrono::system_clock::now();
    auto part2 = part2Results(sensors);
    std::cout << "Part 2 results: " << part2 << "\nTook "
              << static_cast<double>(
                     std::chrono::duration_cast<std::chrono::milliseconds>(
                         std::chrono::system_clock::now() - cTime)
                         .count()) /
                     1000.0
              << " seconds.\n";

    return 0;
}

long manhattanDistance(Point p1, Point p2) {
    return std::abs(p2.x - p1.x) + std::abs(p2.y - p1.y);
}

std::pair<SensorSet, std::unordered_set<Point>>
parseInput(const std::string &fname) {
    std::ifstream inputFile(fname);
    SensorSet set;
    std::unordered_set<Point> beacons;

    std::string line;
    while (std::getline(inputFile, line)) {
        Point sensorPos, beaconPos;
        auto pos = line.find('=');
        std::size_t numChars;
        sensorPos.x = std::stol(line.substr(pos + 1), &numChars);
        pos += numChars;
        pos = line.find('=', pos);
        sensorPos.y = std::stol(line.substr(pos + 1), &numChars);
        pos += numChars;
        pos = line.find('=', pos);
        beaconPos.x = std::stol(line.substr(pos + 1), &numChars);
        pos += numChars;
        pos = line.find('=', pos);
        beaconPos.y = std::stol(line.substr(pos + 1), &numChars);
        beacons.insert(beaconPos);
        set.push(Sensor(sensorPos, beaconPos));
    }

    return std::make_pair(set, beacons);
}

long part1Results(const SensorSet &sensors,
                  const std::unordered_set<Point> &beacons) {
    long total = 0;

    for (Point cPoint(sensors.bounds().first.first, 2000000);
         cPoint.x <= sensors.bounds().first.second; ++cPoint.x) {
        if (!sensors.canBeBeacon(cPoint) && !beacons.contains(cPoint)) {
            total += 1;
        }
    }

    return total;
}

long part2Results(const SensorSet &sensors) {
    const long bindLimit = 4000000;

    std::unordered_set<long> upLines, downLines;

    for (auto &sensor : sensors.set()) {
        upLines.insert(sensor.pos().y - sensor.pos().x + sensor.range() + 1);
        upLines.insert(sensor.pos().y - sensor.pos().x - sensor.range() - 1);
        downLines.insert(sensor.pos().x + sensor.pos().y + sensor.range() + 1);
        downLines.insert(sensor.pos().x + sensor.pos().y - sensor.range() - 1);
    }

    for (const auto &upLine : upLines) {
        for (const auto &downLine : downLines) {
            Point p((downLine - upLine) / 2, (upLine + downLine) / 2);
            if (p.x >= 0 && p.x <= bindLimit && p.y > 0 && p.y <= bindLimit &&
                sensors.canBeBeacon(p)) {
                return 4000000 * p.x + p.y;
            }
        }
    }

    return std::numeric_limits<long>::min();
}

bool operator==(const Point &p1, const Point &p2) {
    return p1.x == p2.x && p1.y == p2.y;
}

Sensor::Sensor(Point pos, long range) : pos_(pos), range_(range) {}

Sensor::Sensor(Point pos, Point beaconPos) : pos_(pos) {
    range_ = manhattanDistance(pos, beaconPos);
}

bool operator==(const Sensor &s1, const Sensor &s2) {
    return s1.pos_ == s2.pos_ && s1.range_ == s2.range_;
}

bool Sensor::isInRange(const Point &pos) const {
    return range_ >= manhattanDistance(pos, pos_);
}

SensorSet::SensorSet() {
    topLeft_ = Point{std::numeric_limits<long>::max(),
                     std::numeric_limits<long>::max()};
    bottomRight_ = Point{std::numeric_limits<long>::min(),
                         std::numeric_limits<long>::min()};
}

void SensorSet::push(const Sensor &sensor) {
    if (sensor.pos().x - sensor.range() < topLeft_.x)
        topLeft_.x = sensor.pos().x - sensor.range();
    if (sensor.pos().x + sensor.range() > bottomRight_.x)
        bottomRight_.x = sensor.pos().x + sensor.range();
    if (sensor.pos().y - sensor.range() < topLeft_.y)
        topLeft_.y = sensor.pos().y - sensor.range();
    if (sensor.pos().y + sensor.range() > bottomRight_.y)
        bottomRight_.y = sensor.pos().y + sensor.range();

    set_.insert(sensor);
}

bool SensorSet::contains(const Sensor &sensor) const {
    return set_.contains(sensor);
}

bool SensorSet::canBeBeacon(const Point &p) const {
    for (auto &sensor : set_) {
        if (sensor.isInRange(p)) {
            return false;
        }
    }

    return true;
}

std::pair<std::pair<long, long>, std::pair<long, long>>
SensorSet::bounds() const {
    return std::make_pair(std::make_pair(topLeft_.x, bottomRight_.x),
                          std::make_pair(topLeft_.y, bottomRight_.y));
}
