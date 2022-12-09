#include "point_set.h"
#include <stdio.h>
#include <stdlib.h>

#define STACK_SIZE_MIN (32)

long pointCompare(struct Point p1, struct Point p2) {
    if (p1.x != p2.x)
        return p1.x - p2.x;
    return p1.y - p2.y;
}

struct PointSetNode *pointSetNodeNew(struct Point val) {
    struct PointSetNode *self = malloc(sizeof(struct PointSetNode));
    if (!self)
        return NULL;

    self->left = NULL;
    self->right = NULL;
    self->val = val;

    return self;
}

void pointSetNodeDelete(struct PointSetNode *self) {
    if (!self)
        return;

    pointSetNodeDelete(self->left);
    pointSetNodeDelete(self->right);

    free(self);
}

struct PointSet *pointSetNew(void) {
    struct PointSet *self = malloc(sizeof(struct PointSet));
    if (!self)
        return NULL;

    self->head = NULL;
    self->size = 0;

    return self;
}

int pointSetInsert(struct PointSet *self, struct Point val) {
    if (!self)
        return 1;

    struct PointSetNode *node = pointSetNodeNew(val);
    if (!node)
        return 1;

    if (!self->head) {
        self->head = node;
        self->size++;
        return 0;
    }

    struct PointSetTraveller *traveller = pointSetTravellerNew(self->head);
    if (!traveller) {
        pointSetNodeDelete(node);
        return 1;
    }

    struct PointSetNode *top = pointSetTravellerPeek(traveller);

    while (1) {
        long cmp = pointCompare(val, top->val);

        if (!cmp) {
            // already in set
            break;
        } else if (cmp < 0) {
            // val < top->val
            if (top->left) {
                pointSetTravellerInsert(traveller, top->left);
            } else {
                top->left = node;
                self->size++;
                break;
            }
        } else {
            // val > top->val
            if (top->right) {
                pointSetTravellerInsert(traveller, top->right);
            } else {
                top->right = node;
                self->size++;
                break;
            }
        }

        top = pointSetTravellerPeek(traveller);
    }

    pointSetTravellerDelete(traveller);

    return 0;
}

int pointSetContains(struct PointSet *self, struct Point val) {
    if (!self)
        return 0;

    int contains = 0;

    struct PointSetTraveller *traveller = pointSetTravellerNew(self->head);
    if (!traveller)
        return 0;

    struct PointSetNode *top = pointSetTravellerPeek(traveller);

    while (1) {
        long cmp = pointCompare(val, top->val);

        if (!cmp) {
            // found it!
            contains = 1;
            break;
        } else if (cmp < 0) {
            // val < top->val
            if (top->left)
                pointSetTravellerInsert(traveller, top->left);
            else
                break;
        } else {
            // val > top->val
            if (top->right)
                pointSetTravellerInsert(traveller, top->right);
            else
                break;
        }

        top = pointSetTravellerPeek(traveller);
    }

    pointSetTravellerDelete(traveller);

    return contains;
}

void pointSetDelete(struct PointSet *self) {
    if (!self)
        return;

    pointSetNodeDelete(self->head);
    free(self);
}

struct PointSetTraveller *pointSetTravellerNew(struct PointSetNode *initNode) {
    if (!initNode)
        return NULL;

    struct PointSetTraveller *self = malloc(sizeof(struct PointSetTraveller));
    if (!self)
        return NULL;

    self->stackSize = STACK_SIZE_MIN;
    self->stack = malloc(sizeof(struct PointSetNode *) * self->stackSize);
    if (!self->stack) {
        free(self);
        return NULL;
    }

    self->stackLength = 1;
    self->stack[0] = initNode;

    return self;
}

int pointSetTravellerInsert(struct PointSetTraveller *self,
                            struct PointSetNode *val) {
    if (self->stackLength == self->stackSize) {
        size_t newStackSize = self->stackSize * 3 / 2;
        struct PointSetNode **newStack = reallocarray(
            self->stack, newStackSize, sizeof(struct PointSetNode *));
        if (!newStack)
            return 1;
        self->stack = newStack;
        self->stackSize = newStackSize;
    }

    self->stack[self->stackLength++] = val;

    return 0;
}

struct PointSetNode *pointSetTravellerPeek(struct PointSetTraveller *self) {
    if (!self)
        return NULL;

    return self->stack[self->stackLength - 1];
}

struct PointSetNode *pointSetTravellerPop(struct PointSetTraveller *self) {
    if (!self)
        return NULL;

    struct PointSetNode *top = self->stack[--(self->stackLength)];

    if (self->stackLength < self->stackSize / 2 &&
        self->stackSize * 2 / 3 >= STACK_SIZE_MIN) {
        size_t newStackSize = self->stackSize * 2 / 3;
        struct PointSetNode **newStack = reallocarray(
            self->stack, newStackSize, sizeof(struct PointSetNode *));
        if (!newStack)
            return top;
        self->stack = newStack;
        self->stackSize = newStackSize;
    }

    return top;
}

void pointSetTravellerDelete(struct PointSetTraveller *self) {
    if (!self)
        return;

    free(self->stack);
    free(self);
}
