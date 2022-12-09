#ifndef AOCD8_POINT_SET_H
#define AOCD8_POINT_SET_H

#include <stddef.h>

struct Point {
    size_t x;
    size_t y;
};

struct PointSetNode {
    struct PointSetNode *left;
    struct PointSetNode *right;

    struct Point val;
};

struct PointSet {
    struct PointSetNode *head;

    size_t size;
};

struct PointSetTraveller {
    size_t stackLength;
    size_t stackSize;

    struct PointSetNode **stack;
};

// Point functions
long pointCompare(struct Point p1, struct Point p2);

// PointSetNode functions

struct PointSetNode *pointSetNodeNew(struct Point val);

void pointSetNodeDelete(struct PointSetNode *self);

// PointSet functions

struct PointSet *pointSetNew(void);

int pointSetInsert(struct PointSet *self, struct Point val);

int pointSetContains(struct PointSet *self, struct Point val);

void pointSetDelete(struct PointSet *self);

// PointSetTraveller functions

struct PointSetTraveller *pointSetTravellerNew(struct PointSetNode *initNode);

int pointSetTravellerInsert(struct PointSetTraveller *self,
                            struct PointSetNode *val);

struct PointSetNode *pointSetTravellerPeek(struct PointSetTraveller *self);

struct PointSetNode *pointSetTravellerPop(struct PointSetTraveller *self);

void pointSetTravellerDelete(struct PointSetTraveller *self);

#endif
