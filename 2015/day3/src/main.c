#include "pointset.h"
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

// Why did I do this in C??????? It's not even good because I used a plain
// binary tree rather than an RB-tree or a hash table. Also, so many copies and
// very few moves, because everyone must own everything so that I can keep track
// of where to free things.

static char *readFile(char *input, size_t *inputLength);
static long numberOfHouses(char *input, size_t inputLength,
                           unsigned int numOfSantas);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <input file>\n", argv[0]);
        return 1;
    }

    size_t inputLength = 0;
    char *input = readFile(argv[1], &inputLength);
    if (!input) {
        fprintf(stderr, "Failed to read from file %s\n", argv[1]);
        return 1;
    }

    printf("Part 1 answer: %ld\n", numberOfHouses(input, inputLength, 1));
    printf("Part 2 answer: %ld\n", numberOfHouses(input, inputLength, 2));

    free(input);

    return 0;
}

static char *readFile(char *input, size_t *inputLength) {
    if (!input)
        return NULL;

    FILE *inputFile = fopen(input, "r");
    if (!inputFile)
        return NULL;

    fseek(inputFile, 0, SEEK_END);
    *inputLength = ftell(inputFile);
    fseek(inputFile, 0, SEEK_SET);

    char *inputText = malloc(*inputLength + 1);
    if (!inputText) {
        fclose(inputFile);
        return NULL;
    }

    if (fread(inputText, *inputLength, 1, inputFile) != 1) {
        free(inputText);
        fclose(inputFile);
        return NULL;
    }

    inputText[*inputLength - 1] = 0;

    fclose(inputFile);

    return inputText;
}

static long numberOfHouses(char *input, size_t inputLength,
                           unsigned int numOfSantas) {
    PointSet *houses = PointSet_new();
    if (!houses)
        return LONG_MIN;

    Point **currentHouses = malloc(numOfSantas * sizeof(Point *));
    if (!currentHouses) {
        PointSet_delete(houses);
        return LONG_MIN;
    }
    for (unsigned int i = 0; i < numOfSantas; ++i) {
        currentHouses[i] = Point_new(0, 0);
        if (!currentHouses[i]) {
            PointSet_delete(houses);
            for (unsigned int j = 0; j < i; ++j) {
                Point_delete(currentHouses[j]);
            }
            return LONG_MIN;
        }
    }

    PointSet_insert(houses, currentHouses[0]);
    for (size_t i = 0; i < inputLength;) {
        for (unsigned int j = 0; j < numOfSantas && i < inputLength; ++j, ++i) {
            switch (input[i]) {
            case '<':
                Point_setX(currentHouses[j], Point_getX(currentHouses[j]) - 1);
                break;

            case '>':
                Point_setX(currentHouses[j], Point_getX(currentHouses[j]) + 1);
                break;

            case '^':
                Point_setY(currentHouses[j], Point_getY(currentHouses[j]) + 1);
                break;

            case 'v':
                Point_setY(currentHouses[j], Point_getY(currentHouses[j]) - 1);
                break;

            default:
                break;
            }
            PointSet_insert(houses, currentHouses[j]);
        }
    }

    for (unsigned int i = 0; i < numOfSantas; ++i) {
        Point_delete(currentHouses[i]);
    }
    free(currentHouses);
    long pointsSize = PointSet_getSize(houses);
    PointSet_delete(houses);
    return pointsSize;
}
