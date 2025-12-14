#ifndef D5CD5C03_8038_4430_AFA8_E7C524919590
#define D5CD5C03_8038_4430_AFA8_E7C524919590

#include <stddef.h>

typedef struct Point Point;

typedef struct PointSet PointSet;

/**
 * @brief Constructor for Point
 *
 * @param x
 * @param y
 * @return Point*
 */
Point *Point_new(long x, long y);

/**
 * @brief Create copy of Point
 *
 * @param other
 * @return Point*
 */
Point *Point_copy(Point *other);

/**
 * @brief Compare two Points
 *
 * This uses memcmp under the hood
 *
 * @param self
 * @param other
 * @return int <0 if self<other, 0 if self==other, >0 if self>other
 */
int Point_compare(Point *self, Point *other);

/**
 * @brief Getter for `x`
 * 
 * @param self 
 * @return long 
 */
long Point_getX(Point *self);

/**
 * @brief Getter for `y`
 * 
 * @param self 
 * @return long 
 */
long Point_getY(Point *self);

/**
 * @brief Setter for `x`
 * 
 * @param self 
 * @param x 
 * @return int 
 */
int Point_setX(Point *self, long x);

/**
 * @brief Setter for `y`
 * 
 * @param self 
 * @param y 
 * @return int 
 */
int Point_setY(Point *self, long y);

/**
 * @brief Destructor for Point
 * 
 * @param self 
 */
void Point_delete(Point *self);

/**
 * @brief Constructor for PointSet
 * 
 * @return PointSet* 
 */
PointSet *PointSet_new(void);

/**
 * @brief Insert /copy/ of Point into PointSet
 * 
 * @param self 
 * @param value COPIED point---does not take ownership
 * @return int 
 */
int PointSet_insert(PointSet *self, Point *value);

/**
 * @brief Check if PointSet contains Point
 * 
 * @param self 
 * @param value 
 * @return int 
 */
int PointSet_contains(PointSet *self, Point *value);

/**
 * @brief Getter for `size`
 * 
 * @param self 
 * @return long 
 */
long PointSet_getSize(PointSet *self);

/**
 * @brief Destructor for PointSet
 * 
 * @param self 
 */
void PointSet_delete(PointSet *self);

#endif // D5CD5C03_8038_4430_AFA8_E7C524919590
