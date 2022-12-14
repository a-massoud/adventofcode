#include <algorithm>
#include <fstream>
#include <iostream>
#include <memory>
#include <utility>
#include <variant>
#include <vector>

// this was actually quite simple.
// It would have been done almost immediately if not for a segfault that I
// assumed was in the parser (and took ages of debugging to figure out was not)
// but was actually in the << operator for streams on PacketLists. Goddamnit.

// there's probably some possible abuse of the templating system that will make
// this able to work in one line but I don't know what it is.
struct PacketList {
    std::vector<std::variant<int, std::unique_ptr<PacketList>>> list;

    friend std::strong_ordering operator<=>(const PacketList &l1,
                                            const PacketList &l2);
    friend bool operator==(const PacketList &l1, const PacketList &l2) {
        return std::is_eq(l1 <=> l2);
    }
};

std::vector<PacketList> parseInput(const std::string &fname);
PacketList listFromString(const std::string &str, std::size_t &end);

std::ostream &
operator<<(std::ostream &ostream,
           const std::variant<int, std::unique_ptr<PacketList>> &item);
std::ostream &operator<<(std::ostream &ostream, const PacketList &list);
std::ostream &operator<<(std::ostream &ostream, std::strong_ordering &cmp);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "I need input\n";
        return 1;
    }

    auto lists = parseInput(argv[1]);

    for (auto &list : lists) {
        std::cout << list << '\n';
    }

    int part1Total = 0;
    for (int i = 0; i < static_cast<int>(lists.size()) - 1; i += 2) {
        bool isInOrder = lists[i] < lists[i + 1];
        int idx = (i / 2) + 1;
        std::cout << "Pair " << idx << " (" << lists[i] << " vs. "
                  << lists[i + 1] << ") in order? "
                  << (isInOrder ? "true" : "false") << '\n';
        if (isInOrder)
            part1Total += idx;
    }
    std::cout << "Part 1 results: " << part1Total << '\n';

    std::size_t end;
    auto div2 = listFromString("[[2]]", end);
    auto div6 = listFromString("[[6]]", end);

    lists.push_back(listFromString("[[2]]", end));
    lists.push_back(listFromString("[[6]]", end));

    std::sort(lists.begin(), lists.end());
    for (auto &list : lists) {
        std::cout << list << '\n';
    }

    long part2Total = 1;
    for (std::size_t i = 0; i < lists.size(); ++i) {
        if (lists[i] == div2 || lists[i] == div6) {
            part2Total *= i + 1;
        }
    }
    std::cout << "Part 2 results: " << part2Total << '\n';

    return 0;
}

std::vector<PacketList> parseInput(const std::string &fname) {
    std::vector<PacketList> lists;

    std::ifstream input(fname);
    if (!input)
        return lists;

    std::string line;
    while (input) {
        std::size_t end;

        if (!std::getline(input, line))
            break;

        lists.push_back(listFromString(line, end));

        if (!std::getline(input, line))
            break;

        lists.push_back(listFromString(line, end));

        std::getline(input, line);
    }

    return lists;
}

PacketList listFromString(const std::string &str, std::size_t &end) {
    PacketList list;
    list.list.clear();

    if (str.length() < 2 || str[0] != '[') {
        end = str.length();
        return list;
    }

    for (std::size_t i = 1; i < str.length() && str[i] != ']';) {
        std::size_t len = 0;
        try {
            list.list.push_back(std::stoi(str.substr(i), &len));
        } catch (std::exception &e) {
            if (str[i] != '[') {
                end = str.length();
                return list;
            }

            auto subList = std::make_unique<PacketList>();
            *subList = listFromString(str.substr(i), len);
            list.list.push_back(std::move(subList));
            ++len;
        }
        i += len;

        if (str[i] == ',')
            i += 1;

        end = i;
    }

    return list;
}

std::ostream &
operator<<(std::ostream &ostream,
           const std::variant<int, std::unique_ptr<PacketList>> &item) {
    if (std::holds_alternative<int>(item))
        ostream << std::get<int>(item);
    else
        ostream << *std::get<std::unique_ptr<PacketList>>(item);

    return ostream;
}

std::ostream &operator<<(std::ostream &ostream, const PacketList &list) {
    ostream << '[';

    for (long i = 0; i < static_cast<long>(list.list.size()) - 1; ++i)
        ostream << list.list[i] << ", ";
    if (list.list.size() != 0)
        ostream << list.list[list.list.size() - 1] << ']';
    else
        ostream << ']';

    return ostream;
}

std::strong_ordering operator<=>(const PacketList &l1, const PacketList &l2) {
    long i;
    for (i = 0; i < static_cast<long>(l1.list.size()) &&
                i < static_cast<long>(l2.list.size());
         ++i) {
        if (std::holds_alternative<std::unique_ptr<PacketList>>(l1.list[i]) &&
            std::holds_alternative<std::unique_ptr<PacketList>>(l2.list[i])) {
            std::strong_ordering cmp =
                *std::get<std::unique_ptr<PacketList>>(l1.list[i]) <=>
                *std::get<std::unique_ptr<PacketList>>(l2.list[i]);

            if (std::is_neq(cmp)) {
                return cmp;
            }
        } else if (std::holds_alternative<int>(l1.list[i]) &&
                   std::holds_alternative<int>(l2.list[i])) {
            int i1 = std::get<int>(l1.list[i]);
            int i2 = std::get<int>(l2.list[i]);
            std::strong_ordering cmp = i1 <=> i2;

            if (std::is_neq(cmp)) {
                return cmp;
            }
        } else {
            PacketList ltmp;
            PacketList rtmp;
            auto &left =
                std::holds_alternative<std::unique_ptr<PacketList>>(l1.list[i])
                    ? *std::get<std::unique_ptr<PacketList>>(l1.list[i])
                    : *[&ltmp, &l1, &i]() {
                          ltmp.list.push_back(std::get<int>(l1.list[i]));
                          return &ltmp;
                      }();
            auto &right =
                std::holds_alternative<std::unique_ptr<PacketList>>(l2.list[i])
                    ? *std::get<std::unique_ptr<PacketList>>(l2.list[i])
                    : *[&rtmp, &l2, &i]() {
                          rtmp.list.push_back(std::get<int>(l2.list[i]));
                          return &rtmp;
                      }();

            std::strong_ordering cmp = left <=> right;

            if (std::is_neq(cmp)) {
                return cmp;
            }
        }
    }

    long l1Size = static_cast<long>(l1.list.size());
    long l2Size = static_cast<long>(l2.list.size());

    // l1 ran out of inputs first
    if (i >= l1Size && i < l2Size)
        return std::strong_ordering::less;

    // l2 ran out first
    if (i >= l2Size && i < l1Size)
        return std::strong_ordering::greater;

    return std::strong_ordering::equal;
}

std::ostream &operator<<(std::ostream &ostream, std::strong_ordering &cmp) {
    if (std::is_lt(cmp)) {
        ostream << "less";
    } else if (std::is_gt(cmp)) {
        ostream << "greater";
    } else {
        ostream << "equal";
    }

    return ostream;
}
