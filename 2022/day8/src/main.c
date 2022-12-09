#include "point_set.h"
#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// I decided to do today's in C out of... masochism? I really don't know.
// It was fun, but it took forever, and not least of that forever was debugging
// the set implementation. "Trees are easy" I said, and then royally fucked it
// so many times. It works now though! I really expected to need that
// pointSetContains() function in part 2 but guess not.

char **processInput(char *inputBuf, size_t inputBufSize, long *numRows,
                    long *numCols);
long part1Results(char **input, long width, long height,
                  struct PointSet *visible);
long calcScenicScore(char **input, long width, long height, long cx, long cy);
long part2Results(char **input, long width, long height);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("give me some input. gimme gimme\n");
        return 1;
    }

    FILE *inputFile = fopen(argv[1], "r");
    if (!inputFile) {
        fprintf(stderr, "Failed to open file \"%s\": %s\n", argv[1],
                strerror(errno));
        return 1;
    }

    fseek(inputFile, 0, SEEK_END);
    size_t inputSize = ftell(inputFile);
    fseek(inputFile, 0, SEEK_SET);

    char *inputBuf = malloc(inputSize * sizeof(char) + 1);
    if (!inputBuf) {
        fprintf(stderr, "Failed to allocate input buffer: %s\n",
                strerror(errno));
        fclose(inputFile);
        return 1;
    }
    fread(inputBuf, inputSize, 1, inputFile);
    fclose(inputFile);

    long numRows, numCols;
    char **input = processInput(inputBuf, inputSize, &numRows, &numCols);
    if (!input) {
        fprintf(stderr, "Failed to create input array: %s\n", strerror(errno));
        return 1;
    }

    struct PointSet *visible = pointSetNew();
    if (!visible) {
        fprintf(stderr, "Whoops failed to allocate for a PointSet: %s\n",
                strerror(errno));
        return 1;
    }

    long part1 = part1Results(input, numRows, numCols, visible);
    long part2 = part2Results(input, numRows, numCols);

    printf("Part 1 results: %ld\nPart 2 results: %ld\n", part1, part2);

    pointSetDelete(visible);
    free(inputBuf);
    free(input);
    return 0;
}

char **processInput(char *inputBuf, size_t inputBufSize, long *numRows,
                    long *numCols) {
    *numCols = 0;
    *numRows = 0;
    for (size_t i = 0; i < inputBufSize; ++i) {
        if (inputBuf[i] == '\n') {
            ++(*numRows);
            if (*numCols == 0)
                *numCols = i;
        }
    }
    if (inputBuf[inputBufSize - 1] != '\n')
        ++(*numRows);

    char **input = malloc(*numRows * sizeof(char *));
    if (!input)
        return NULL;

    for (long i = 0; i < *numRows; ++i) {
        input[i] = inputBuf + i * (*numCols + 1);
    }

    return input;
}

long part1Results(char **input, long width, long height,
                  struct PointSet *visible) {
    // rows from left
    for (long y = 0; y < height; ++y) {
        char minHeight = 0;
        for (long x = 0; x < width; ++x) {
            if (input[y][x] > minHeight) {
                minHeight = input[y][x];
                struct Point p = {x, y};
                pointSetInsert(visible, p);
            }
        }
    }

    // rows from right
    for (long y = 0; y < height; ++y) {
        char minHeight = 0;
        for (long x = width - 1; x >= 0; --x) {
            if (input[y][x] > minHeight) {
                minHeight = input[y][x];
                struct Point p = {x, y};
                pointSetInsert(visible, p);
            }
        }
    }

    // cols from top
    for (long x = 0; x < width; ++x) {
        char minHeight = 0;
        for (long y = 0; y < height; ++y) {
            if (input[y][x] > minHeight) {
                minHeight = input[y][x];
                struct Point p = {x, y};
                pointSetInsert(visible, p);
            }
        }
    }

    // cols from bottom
    for (long x = 0; x < width; ++x) {
        char minHeight = 0;
        for (long y = height - 1; y >= 0; --y) {
            if (input[y][x] > minHeight) {
                minHeight = input[y][x];
                struct Point p = {x, y};
                pointSetInsert(visible, p);
            }
        }
    }

    long num = visible->size;

    return num;
}

long calcScenicScore(char **input, long width, long height, long cx, long cy) {
    if (cx == 0 || cy == 0 || cx == width - 1 || cy == width - 1)
        return 0;

    long scores[4] = {1, 1, 1, 1};

    // down
    for (long y = cy + 1; y < height - 1 && input[y][cx] < input[cy][cx]; ++y)
        ++scores[0];

    // up
    for (long y = cy - 1; y > 0 && input[y][cx] < input[cy][cx]; --y)
        ++scores[1];

    // right
    for (long x = cx + 1; x < width - 1 && input[cy][x] < input[cy][cx]; ++x)
        ++scores[2];

    // left
    for (long x = cx - 1; x > 0 && input[cy][x] < input[cy][cx]; --x)
        ++scores[3];

    return scores[0] * scores[1] * scores[2] * scores[3];
}

long part2Results(char **input, long width, long height) {
    long maxScenic = 0;

    for (long y = 0; y < height; ++y) {
        for (long x = 0; x < width; ++x) {
            long score = calcScenicScore(input, width, height, x, y);
            if (score > maxScenic)
                maxScenic = score;
        }
    }

    return maxScenic;
}
