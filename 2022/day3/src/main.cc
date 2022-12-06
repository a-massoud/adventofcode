#include <iostream>
#include <vector>
#include <fstream>
#include <array>
#include <unordered_set>

int getPriorityFromItem(char item);
int getPriorityDay1(const std::string &rucksack);
int getPriorityDay2(const std::array<std::string, 3> &group);

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

    std::string inputLine;
    std::vector<std::string> lines;
    while (std::getline(input, inputLine) && inputLine != "")
        lines.push_back(inputLine);

    int totalPriority = 0;
    for (auto line: lines)
        totalPriority += getPriorityDay1(line);

    std::cout << "Priority: " << totalPriority << "\n";

    totalPriority = 0;
    for (std::size_t i = 0; i < lines.size(); i += 3)
        totalPriority += getPriorityDay2({lines[i], lines[i + 1], lines[i + 2]});

    std::cout << "Group priority: " << totalPriority << "\n";

    return 0;
}

int getPriorityFromItem(char item) {
    int priority = 0;
    if ('a' <= item && item <= 'z') {
        priority = item - 'a' + 1;
    } else if ('A' <= item && item <= 'Z') {
        priority = item - 'A' + 27;
    }

    return priority;
}

// could probably use some hashset stuff, but that would probably only speed
// things up if we had much, much longer strings
int getPriorityDay1(const std::string &rucksack) {
    char val = 0;
    for (auto i = rucksack.begin(); i != rucksack.begin() + rucksack.length() / 2; ++i) {
        for (auto j = rucksack.begin() + rucksack.length() / 2; j != rucksack.end(); ++j) {
            if (*i == *j) {
                val = *i;
                break;
            }
        }
    }

    return getPriorityFromItem(val);
}

// yay hashset stuff!
int getPriorityDay2(const std::array<std::string, 3> &group) {
    const std::string &elf0 = group[0];

    std::unordered_set<char> elf1, elf2;
    for (char item : group[1])
        elf1.insert(item);
    for (char item : group[2])
        elf2.insert(item);

    char commonItem = 0;
    for (char item : elf0) {
        if (elf1.contains(item) && elf2.contains(item)) {
            commonItem = item;
            break;
        }
    }
    if (!commonItem) {
        std::cerr << "No common item found\n";
        return -1;
    }

    return getPriorityFromItem(commonItem);
}
