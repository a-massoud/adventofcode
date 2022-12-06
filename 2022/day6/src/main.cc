#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>

// not the best solution by far...
// but not the worst.
// I could use bitwise stuff to track seen letters but that seems like a lot of
// work for very little benefit.

const std::string readInput(const std::string &fname);
int getNumDistinct(const std::string &input, int numDistinct);

int main(int argc, char *argv[]) {
    if (argc < 2)
        return 1;

    std::string input = readInput(argv[1]);

    std::cout << "Part 1 answer: " << getNumDistinct(input, 4) << ".\n";
    std::cout << "Part 2 answer: " << getNumDistinct(input, 14) << ".\n";

    return 0;
}

const std::string readInput(const std::string &fname) {
    std::ifstream inputFile(fname);
    if (!inputFile.good())
        throw std::runtime_error(
            (std::stringstream("Failed to open ") << fname << " for reading.")
                .str());

    std::string line;
    if (!std::getline(inputFile, line))
        throw std::runtime_error("Failed to read line.");

    return line;
}

int getNumDistinct(const std::string &input, int numDistinct) {
    --numDistinct;
    for (int i = numDistinct; i < (int)input.length(); ++i) {
        bool found = false;
        for (int j = i; j >= i - numDistinct && !found; --j) {
            for (int k = j - 1; k >= i - numDistinct && !found; --k) {
                if (input[j] == input[k])
                    found = true;
            }
        }
        if (!found)
            return i + 1;
    }
    return -1;
}
