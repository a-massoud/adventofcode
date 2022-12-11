#ifndef ADVENTOFCODE_2022_DAY11_MONKEY_H_8DH2I
#define ADVENTOFCODE_2022_DAY11_MONKEY_H_8DH2I

#include <cstdint>
#include <deque>
#include <utility>
#include <vector>

class Monkey {
  private:
    // items the monkey currently has in its inventory
    std::deque<std::intmax_t> inventory_;

    // positive: add, zero: square, negative: multiply
    std::intmax_t operation_;

    // value to test against
    std::intmax_t testVal_;

    // our true and false monkeys, indexed in the global monkey list
    std::size_t trueMonkey_;
    std::size_t falseMonkey_;

    // count of all inspected items
    std::intmax_t inspectedCount_;

    // catch from other monkey
    void catchItem(std::intmax_t item);

    // global list of monkeys
    static std::vector<Monkey *> monkeyList_;

    // global mod value
    static std::intmax_t modVal_;

  public:
    Monkey(std::vector<std::intmax_t> initialInventory, std::intmax_t operation,
           std::intmax_t testVal, std::size_t trueMonkey,
           std::size_t falseMonkey);
    Monkey(const Monkey &other);

    ~Monkey();

    std::intmax_t inspectedCount() const { return inspectedCount_; }

    static std::vector<Monkey *> &monkeyList() { return monkeyList_; }

    void step(bool shouldDiv);

    void print();
};

#endif
