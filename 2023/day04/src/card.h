#ifndef CARD_H_5P7PWGUE
#define CARD_H_5P7PWGUE

#include <unordered_set>
#include <vector>

struct Card {
    int n_copies;
    std::vector<int> numbers;
    std::unordered_set<int> winners;

    Card(){};
    Card(const std::vector<int> &numbers,
         const std::unordered_set<int> &winners)
        : n_copies(1), numbers(numbers), winners(winners) {}

    int score() const noexcept;
    int n_matches() const noexcept;
};

#endif /* CARD_H_5P7PWGUE */
