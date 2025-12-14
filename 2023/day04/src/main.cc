#include "card.h"
#include <cstddef>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <unordered_set>
#include <vector>

// This one was honestly pretty easy. I do think there's probably a better
// parsing function than the one I wound up writing though.

std::vector<Card> read_input(const std::string &fname);
long part1(const std::vector<Card> &input);
long part2(std::vector<Card> &&input);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>\n";
        return 1;
    }

    std::vector<Card> input = read_input(argv[1]);

    std::cout << "Part 1 results: " << part1(input) << "\n";
    std::cout << "Part 2 results: " << part2(std::vector(input)) << "\n";

    return 0;
}

std::vector<Card> read_input(const std::string &fname) {
    std::vector<Card> values;

    std::ifstream input(fname);
    if (!input.good())
        return values;

    std::string line;
    while (std::getline(input, line)) {
        std::size_t card_pos = line.find(':');
        if (card_pos == std::string::npos)
            throw std::runtime_error("Could not find `:` in `" + line + "`");
        std::size_t split_pos = line.find('|');
        if (split_pos == std::string::npos)
            throw std::runtime_error("Could not find `|` in `" + line + "`");

        std::string numbers_line =
            line.substr(card_pos + 1, split_pos - card_pos - 1);
        std::string winners_line = line.substr(split_pos + 1);

        // get numbers
        std::vector<int> numbers;
        while (true) {
            try {
                std::size_t pos;
                numbers.push_back(std::stoi(numbers_line, &pos));
                numbers_line = numbers_line.substr(pos);
            } catch (std::invalid_argument &e) {
                break;
            }
        }

        // get winners
        std::unordered_set<int> winners;
        while (true) {
            try {
                std::size_t pos;
                winners.insert(std::stoi(winners_line, &pos));
                winners_line = winners_line.substr(pos);
            } catch (std::invalid_argument &e) {
                break;
            }
        }

        values.push_back(Card(numbers, winners));
    }

    return values;
}

long part1(const std::vector<Card> &input) {
    long total = 0;

    for (auto &card : input)
        total += card.score();

    return total;
}

long part2(std::vector<Card> &&input) {
    long total = 0;

    for (auto card = input.begin(); card != input.end(); ++card) {
        total += card->n_copies;

        int matches = card->n_matches();
        for (int i = 1; i <= matches && (card + i) != input.end(); ++i)
            (card + i)->n_copies += card->n_copies;
    }

    return total;
}
