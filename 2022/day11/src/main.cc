#include "monkey.h"
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>

std::vector<Monkey> parseInput(std::string fname);
std::intmax_t part1Result(std::vector<Monkey> &monkeys);
std::intmax_t part2Result(std::vector<Monkey> &monkeys);

// Ok this one's actually awful, I should really only be keeping track of the
// monkeys in only one place, but I didn't, so here we are.

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "gimme some input\n";
        return 1;
    }

    std::vector<Monkey> monkeys = parseInput(argv[1]);

    for (auto monkey : monkeys)
        monkey.print();

    // double check that they agree cus they don't always
    std::cout << "\nMonkey::monkeyList says:\n";
    for (auto monkey : Monkey::monkeyList())
        monkey->print();

    // no easy way to reset, just comment out whichever one you don't want.
    // should've probably done better
    // std::cout << "\nPart 1 results: " << part1Result(monkeys) << "\n";

    std::cout << "\nPart 2 results: " << part2Result(monkeys) << "\n";
}

// this parsing function is just awful but it works, and it doesn't feel like
// I'm cheating by hard-coding it LOL
std::vector<Monkey> parseInput(std::string fname) {
    std::fstream inputFile(fname);
    if (!inputFile)
        return std::vector<Monkey>();

    std::vector<Monkey> monkeys;

    std::string line;
    while (std::getline(inputFile, line)) {
        if (!line.starts_with("Monkey "))
            continue;

        std::vector<long> inventory;
        std::intmax_t operation = 0;
        std::intmax_t testVal = 0;
        std::size_t trueMonkey = 0;
        std::size_t falseMonkey = 0;

        while (std::getline(inputFile, line) && line != "") {
            // trim whitespace
            line.erase(line.begin(),
                       std::find_if(line.begin(), line.end(),
                                    [](char c) { return !std::isspace(c); }));
            auto pos = line.find(':') + 1;
            std::string lookingFor = line.substr(0, pos);
            line.erase(0, pos);

            if (lookingFor == "Starting items:") {
                do {
                    pos = line.find(' ');
                    if (!(pos == std::string::npos)) {
                        line.erase(0, pos + 1);
                        inventory.push_back(std::stol(line));
                    }
                } while (pos != std::string::npos);
            } else if (lookingFor == "Operation:") {
                // delete everything except actual operation
                line.erase(line.begin(),
                           std::find_if(line.begin(), line.end(), [](char c) {
                               return c == '*' || c == '+';
                           }));
                if (line == "* old")
                    operation = 0;
                else if (line[0] == '*')
                    operation = -1 * std::stol(line.substr(2));
                else
                    operation = std::stol(line.substr(2));
            } else if (lookingFor == "Test:") {
                testVal = std::stol(line.substr(std::distance(
                    line.begin(),
                    std::find_if(line.begin(), line.end(), [](char c) {
                        return '0' <= c && c <= '9';
                    }))));
            } else if (lookingFor == "If true:") {
                std::string substr = line.substr(std::distance(
                    line.begin(),
                    std::find_if(line.begin(), line.end(),
                                 [](char c) { return '0' <= c && c <= '9'; })));
                trueMonkey = std::stoul(substr);
            } else if (lookingFor == "If false:") {
                falseMonkey = std::stoul(line.substr(std::distance(
                    line.begin(),
                    std::find_if(line.begin(), line.end(), [](char c) {
                        return '0' <= c && c <= '9';
                    }))));
            }
        }

        monkeys.push_back(
            Monkey(inventory, operation, testVal, trueMonkey, falseMonkey));
    }

    // fix Monkey::monkeyList
    Monkey::monkeyList().clear();
    for (auto &monkey : monkeys)
        Monkey::monkeyList().push_back(&monkey);

    return monkeys;
}

std::intmax_t part1Result(std::vector<Monkey> &monkeys) {
    for (int i = 0; i < 20; ++i) {
        std::cout << "\nRound " << i << "\n";
        for (std::size_t i = 0; i < monkeys.size(); ++i) {
            std::cout << "  Monkey " << i << ":\n";
            monkeys[i].step(true);
        }
    }

    std::vector<long> activity;
    for (auto &monkey : monkeys) {
        activity.push_back(monkey.inspectedCount());
    }

    std::cout << "Our unsorted activity is:\n    ";
    for (auto i : activity)
        std::cout << i << ", ";
    std::cout << "\n";
    std::sort(activity.begin(), activity.end(), std::greater<long>());

    return activity[0] * activity[1];
}

std::intmax_t part2Result(std::vector<Monkey> &monkeys) {
    for (int i = 0; i < 10000; ++i) {
        std::cout << "\nRound " << i << "\n";
        for (std::size_t i = 0; i < monkeys.size(); ++i) {
            std::cout << "  Monkey " << i << ":\n";
            monkeys[i].step(false);
        }
    }

    std::vector<long> activity;
    for (auto &monkey : monkeys) {
        activity.push_back(monkey.inspectedCount());
    }

    std::cout << "Our unsorted activity is:\n    ";
    for (auto i : activity)
        std::cout << i << ", ";
    std::cout << "\n";
    std::sort(activity.begin(), activity.end(), std::greater<long>());

    return activity[0] * activity[1];
}
