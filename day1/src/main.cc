#include <algorithm>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <vector>

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

    std::vector<int> elves;
    while (!input.eof()) {
        int thisElfTotal = 0;
        std::string line;
        while (std::getline(input, line) && line.length() != 0) {
            try {
                thisElfTotal += std::stoi(line);
            } catch (std::invalid_argument &e) {
                std::cerr << "Failed to convert \"" << line
                          << "\": " << e.what() << "\n";
            } catch (std::out_of_range &e) {
                std::cerr << line << " out of range: " << e.what() << "\n";
            }
        }

        elves.push_back(thisElfTotal);
    }

    std::sort(elves.begin(), elves.end(), std::greater<int>());

    std::cout << "1: " << elves[0] << "\n2: " << elves[1] << "\n3: " << elves[2] << "\nTotal: " << elves[0] + elves[1] + elves[2] << "\n";
}
