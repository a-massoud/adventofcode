#include <limits.h>
#include <pointset.h>
#include <stdlib.h>
#include <string.h>

typedef struct PointSetNode PointSetNode;

struct Point {
    long x;
    long y;
};

struct PointSetNode {
    Point *val;
    PointSetNode *left;
    PointSetNode *right;
};

struct PointSet {
    size_t size;
    PointSetNode *head;
};

/**
 * @brief Constructor for PointSetNode
 *
 * @param val
 * @return PointSetNode*
 */
static PointSetNode *PointSetNode_new(Point *val);

/**
 * @brief Destructor for PointSetNode
 *
 * @param self
 */
static void PointSetNode_delete(PointSetNode *self);

Point *Point_new(long x, long y) {
    Point *self = malloc(sizeof(Point));
    if (!self)
        return NULL;

    self->x = x;
    self->y = y;

    return self;
}

Point *Point_copy(Point *other) {
    if (!other)
        return NULL;

    Point *self = malloc(sizeof(Point));
    if (!self)
        return NULL;

    self->x = other->x;
    self->y = other->y;

    return self;
}

int Point_compare(Point *self, Point *other) {
    // It doesn't actually matter if the ordering is human-readable, it just
    // matters that it is strict, and can determine when two are equal.
    return memcmp(self, other, sizeof(Point));
}

long Point_getX(Point *self) {
    if (!self)
        return LONG_MAX;
    return self->x;
}

long Point_getY(Point *self) {
    if (!self)
        return LONG_MIN;
    return self->y;
}

int Point_setX(Point *self, long x) {
    if (!self)
        return 1;
    self->x = x;
    return 0;
}

int Point_setY(Point *self, long y) {
    if (!self)
        return 1;
    self->y = y;
    return 0;
}

void Point_delete(Point *self) { free(self); }

static PointSetNode *PointSetNode_new(Point *val) {
    if (!val)
        return NULL;

    PointSetNode *self = malloc(sizeof(PointSetNode));
    if (!self)
        return NULL;

    self->val = val;
    self->left = NULL;
    self->right = NULL;

    return self;
}

static void PointSetNode_delete(PointSetNode *self) {
    if (!self)
        return;

    Point_delete(self->val);
    if (self) {
        PointSetNode_delete(self->left);
        PointSetNode_delete(self->right);
    }
    free(self);
}

PointSet *PointSet_new(void) {
    PointSet *self = malloc(sizeof(PointSet));
    if (!self)
        return NULL;

    self->head = NULL;
    self->size = 0;

    return self;
}

int PointSet_insert(PointSet *self, Point *value) {
    if (!self)
        return 1;
    if (!value)
        return 1;

    if (!self->head) {
        self->head = PointSetNode_new(Point_copy(value));
        if (!self->head)
            return 1;
        ++self->size;
        return 0;
    }

    PointSetNode *currentNode = self->head;
    PointSetNode *parentNode = NULL;
    while (currentNode) {
        parentNode = currentNode;
        int cmp = Point_compare(value, currentNode->val);
        if (cmp < 0) {
            currentNode = currentNode->left;
        } else if (cmp > 0) {
            currentNode = currentNode->right;
        } else {
            return 0;
        }
    }

    PointSetNode **dest = NULL;
    int cmp = Point_compare(value, parentNode->val);
    if (cmp < 0) {
        dest = &parentNode->left;
    } else {
        dest = &parentNode->right;
    }

    *dest = PointSetNode_new(Point_copy(value));
    if (!*dest) {
        return 1;
    }
    ++self->size;

    return 0;
}

int PointSet_contains(PointSet *self, Point *value) {
    if (!self || !value || !self->head)
        return 0;

    PointSetNode *currentNode = self->head;
    while (currentNode) {
        int cmp = Point_compare(value, currentNode->val);
        if (cmp < 0) {
            currentNode = currentNode->left;
        } else if (cmp > 0) {
            currentNode = currentNode->right;
        } else {
            return 1;
        }
    }

    return 0;
}

long PointSet_getSize(PointSet *self) {
    return self->size;
}

void PointSet_delete(PointSet *self) {
    if (!self)
        return;
    PointSetNode_delete(self->head);
    free(self);
}
