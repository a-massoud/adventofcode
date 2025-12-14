#include "circuit.hh"
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>

Circuit::Circuit() = default;

Circuit::Circuit(const std::vector<std::string> &lines) {
    for (const auto &line : lines)
        insert_wire(line);
}

void Circuit::insert_wire(const std::string &source_line) {
    wire_v_cache_.clear();

    std::vector<std::string> split_line;
    {
        std::stringstream line_stream(source_line);
        std::string line;
        while (std::getline(line_stream, line, ' '))
            split_line.push_back(line);
    }

    if (!(3 <= split_line.size() && split_line.size() <= 5))
        throw std::runtime_error("failed to parse string `" + source_line +
                                 "`: incorrect word count");

    if (split_line.size() == 3) {
        // format `x -> y`
        std::string wire_a = split_line[0];
        if (std::isdigit(wire_a[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_a);
            wire_a = "_" + wire_a;
            wires_[wire_a] = [=]() { return val; };
        }

        wires_[split_line[2]] = [=]() { return value_of(wire_a); };
    } else if (split_line.size() == 4 && split_line[0] == "NOT") {
        std::string wire_a = split_line[1];
        if (std::isdigit(wire_a[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_a);
            wire_a = "_" + wire_a;
            wires_[wire_a] = [=]() { return val; };
        }

        wires_[split_line[3]] = [=]() { return ~value_of(wire_a); };
    } else if (split_line.size() == 5 && split_line[1] == "AND") {
        std::string wire_a = split_line[0];
        if (std::isdigit(wire_a[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_a);
            wire_a = "_" + wire_a;
            wires_[wire_a] = [=]() { return val; };
        }

        std::string wire_b = split_line[2];
        if (std::isdigit(wire_b[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_b);
            wire_b = "_" + wire_b;
            wires_[wire_b] = [=]() { return val; };
        }

        wires_[split_line[4]] = [=]() { return value_of(wire_a) & value_of(wire_b); };
    } else if (split_line.size() == 5 && split_line[1] == "OR") {
        std::string wire_a = split_line[0];
        if (std::isdigit(wire_a[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_a);
            wire_a = "_" + wire_a;
            wires_[wire_a] = [=]() { return val; };
        }

        std::string wire_b = split_line[2];
        if (std::isdigit(wire_b[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_b);
            wire_b = "_" + wire_b;
            wires_[wire_b] = [=]() { return val; };
        }

        wires_[split_line[4]] = [=]() { return value_of(wire_a) | value_of(wire_b); };
    } else if (split_line.size() == 5 && split_line[1] == "RSHIFT") {
        std::string wire_a = split_line[0];
        if (std::isdigit(wire_a[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_a);
            wire_a = "_" + wire_a;
            wires_[wire_a] = [=]() { return val; };
        }

        std::string wire_b = split_line[2];
        if (std::isdigit(wire_b[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_b);
            wire_b = "_" + wire_b;
            wires_[wire_b] = [=]() { return val; };
        }

        wires_[split_line[4]] = [=]() { return value_of(wire_a) >> value_of(wire_b); };
    } else if (split_line.size() == 5 && split_line[1] == "LSHIFT") {
        std::string wire_a = split_line[0];
        if (std::isdigit(wire_a[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_a);
            wire_a = "_" + wire_a;
            wires_[wire_a] = [=]() { return val; };
        }

        std::string wire_b = split_line[2];
        if (std::isdigit(wire_b[0])) {
            // moving a constant in
            uint16_t val = std::stoi(wire_b);
            wire_b = "_" + wire_b;
            wires_[wire_b] = [=]() { return val; };
        }

        wires_[split_line[4]] = [=]() { return value_of(wire_a) << value_of(wire_b); };
    } else {
        throw std::runtime_error("failed to parse string `" + source_line +
                                 "`: invalid operation");
    }
}

uint16_t Circuit::value_of(const std::string &name) {
    if (wire_v_cache_.count(name)) {
        return wire_v_cache_.at(name);
    } else {
        uint16_t val = wires_.at(name)();
        wire_v_cache_[std::string(name)] = val;
        return val;
    }
}
