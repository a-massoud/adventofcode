#ifndef CIRCUIT_HH_GHR8KBBX
#define CIRCUIT_HH_GHR8KBBX

#include <functional>
#include <string>
#include <cstdint>
#include <unordered_map>
#include <variant>
#include <vector>

class Circuit {
  private:
    std::unordered_map<std::string, std::function<uint16_t()>> wires_;
    std::unordered_map<std::string, uint16_t> wire_v_cache_;

  public:
    Circuit();
    Circuit(const std::vector<std::string> &lines);
    void insert_wire(const std::string &source_line);
    uint16_t value_of(const std::string &name);
};

#endif /* CIRCUIT_HH_GHR8KBBX */
