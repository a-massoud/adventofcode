#include "card.h"

int Card::score() const noexcept {
    int running_score = 0;

    for (auto &number : numbers) {
        if (winners.contains(number)) {
            if (running_score != 0)
                running_score *= 2;
            else
                running_score = 1;
        }
    }

    return running_score;
}

int Card::n_matches() const noexcept {
    int matches = 0;

    for (auto &number : numbers) {
        if (winners.contains(number))
            ++matches;
    }

    return matches;
}
