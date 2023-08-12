// Not too bad once I:
//   1. actually understood the assignment
//   2. got past the absolutely stupid compiler errors caused by const member
//      functions
// This is only fast because of the caching, which is also why I don't assign
// `circuit.value_of("a")` to a local variable in `main()`

#include "circuit.hh"
#include <exception>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <vector>

std::vector<std::string> read_lines(const std::string &input);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>\n";
        return 1;
    }

    std::vector<std::string> input = read_lines(argv[1]);

    Circuit circuit(input);
    std::cout << "Circuit has been built.\n";

    std::cout << "Part 1 answer: " << circuit.value_of("a") << "\n";

    circuit.insert_wire(std::to_string(circuit.value_of("a")) + " -> b");
    std::cout << "Part 1 answer: " << circuit.value_of("a") << "\n";

    return 0;
}

std::vector<std::string> read_lines(const std::string &input) {
    std::ifstream input_file(input);
    if (!input_file.good()) {
        throw std::runtime_error("failed to open file `" + input +
                                 "` for reading");
    }

    std::vector<std::string> lines;
    std::string line;
    while (std::getline(input_file, line)) {
        lines.push_back(line);
    }

    return lines;
}
