#ifndef ADVENTOFCODE_2022_DAY12_UTIL_H_FLEQHWER
#define ADVENTOFCODE_2022_DAY12_UTIL_H_FLEQHWER

struct Point {
    int x;
    int y;
};

inline bool operator==(const Point &a, const Point &b) {
    return a.x == b.x && a.y == b.y;
}

#endif
