#include "monkey.h"
#include <algorithm>
#include <iostream>
#include <sstream>

std::vector<Monkey *> Monkey::monkeyList_;
std::intmax_t Monkey::modVal_ = 1;

Monkey::Monkey(std::vector<std::intmax_t> initialInventory,
               std::intmax_t operation, std::intmax_t testVal,
               std::size_t trueMonkey, std::size_t falseMonkey)
    : operation_(operation), testVal_(testVal), trueMonkey_(trueMonkey),
      falseMonkey_(falseMonkey), inspectedCount_(0) {
    for (std::intmax_t item : initialInventory) {
        inventory_.push_back(item);
    }

    monkeyList_.push_back(this);

    if (modVal_ % testVal_)
        modVal_ *= testVal_;
}

Monkey::Monkey(const Monkey &other)
    : inventory_(other.inventory_), operation_(other.operation_),
      testVal_(other.testVal_), trueMonkey_(other.trueMonkey_),
      falseMonkey_(other.falseMonkey_), inspectedCount_(other.inspectedCount_) {
    monkeyList_.push_back(this);
}

Monkey::~Monkey() {
    monkeyList_.erase(std::find(monkeyList_.begin(), monkeyList_.end(), this));
}

void Monkey::catchItem(std::intmax_t item) { inventory_.push_back(item); }

void Monkey::step(bool shouldDiv) {
    while (!inventory_.empty()) {
        std::intmax_t item = *(inventory_.begin());
        ++inspectedCount_;
        std::cout << "    Monkey inspects item with worry value " << item
                  << "\n";

        item %= modVal_;
        std::cout << "      We mod this by " << modVal_
                  << " to keep it sane, to " << item << "\n";

        if (operation_ > 0) {
            item += operation_;
            std::cout << "      This adds " << operation_
                      << " to it, making it now " << item << "\n";
        } else if (operation_ < 0) {
            item *= operation_ * -1;
            std::cout << "      This multiplies it by " << operation_ * -1
                      << ", making it now " << item << "\n";
        } else {
            item *= item;
            std::cout << "      This squares it, making it now " << item
                      << "\n";
        }

        if (shouldDiv) {
            item /= 3;
            std::cout << "      The monkey gets bored with it, dividing it by "
                         "three to "
                      << item << "\n";
        }

        if (item % testVal_ == 0) {
            std::cout << "      This is divisible by " << testVal_
                      << ", so the monkey throws it to " << trueMonkey_ << "\n";
            monkeyList_[trueMonkey_]->catchItem(item);
        } else {
            std::cout << "      This is not divisible by " << testVal_
                      << ", so the monkey throws it to " << falseMonkey_
                      << "\n";
            monkeyList_[falseMonkey_]->catchItem(item);
        }
        inventory_.pop_front();
    }
}

void Monkey::print() {
    std::stringstream items;
    for (auto item : inventory_) {
        items << item << ", ";
    }

    std::cout << "Monkey:\n  Starting items: " << items.str()
              << "\n  Operation: " << operation_ << "\n  Test: " << testVal_
              << "\n  True Monkey: " << trueMonkey_
              << "\n  False Monkey: " << falseMonkey_ << "\n";
}
