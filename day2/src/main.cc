#include <fstream>
#include <iostream>
#include <stdexcept>

int getRoundScore(char opponent, char player);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "You know better\n";
        return 1;
    }

    std::ifstream input(argv[1]);
    if (!input.is_open()) {
        std::cerr << "Failed to open " << argv[1] << ".\n";
        return 1;
    }

    std::string line;
    int totalScore = 0;
    while (!input.eof()) {
        std::getline(input, line);
        if (line.length() < 3)
            continue;

        totalScore += getRoundScore(line[0], line[2]);
    }

    std::cout << "Score: " << totalScore << "\n";

    return 0;
}

int getRoundScore(char opponent, char player) {
    int score = 0;

    switch (opponent) {
    // opponent plays rock
    case 'A':
        switch (player) {
        // we lose (scissors)
        case 'X':
            score = 3 + 0;
            break;

        // we draw (rock)
        case 'Y':
            score = 1 + 3;
            break;

        // we win (paper)
        case 'Z':
            score = 2 + 6;
            break;

        default:
            break;
        }
        break;

    // opponent plays paper
    case 'B':
        switch (player) {
        // we lose (rock)
        case 'X':
            score = 1 + 0;
            break;

        // we draw (paper)
        case 'Y':
            score = 2 + 3;
            break;

        // we win (scissors)
        case 'Z':
            score = 3 + 6;
            break;

        default:
            break;
        }
        break;

    // opponent plays scissors
    case 'C':
        switch (player) {
        // we lose (paper)
        case 'X':
            score = 2 + 0;
            break;

        // we draw (scissors)
        case 'Y':
            score = 3 + 3;
            break;

        // we win (rock)
        case 'Z':
            score = 1 + 6;
            break;

        // something went wrong
        default:
            break;
        }
        break;

    // something went wrong
    default:
        break;
    }

    return score;
}
