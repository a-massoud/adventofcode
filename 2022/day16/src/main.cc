// OK, I really was stumped by this one, quit on it, and only came back to it
// months later. This was hard. This solution only works for actual input on
// part 2 because the sample input is too small. The optimal solution for part 2
// is that both the person and the elephant take optimal non-overlapping routes,
// and when the best route doesn't visit all nodes that is trivial to compute.

#include <fstream>
#include <iostream>
#include <limits>
#include <map>
#include <regex>
#include <set>
#include <vector>

struct Valve {
    int Rate;
    std::set<std::string> Tunnels;
    bool Open;

    Valve() : Rate(0), Tunnels(), Open(false) {}
    Valve(std::set<std::string> &Tunnels, int Rate)
        : Rate(Rate), Tunnels(Tunnels), Open(false){};
};

enum class InTransitValue { NONE, PERSON, ELEPHANT };

std::map<std::string, Valve> readInput(std::string Path);
long partOne(std::map<std::string, Valve> &Valves);
std::pair<std::vector<std::string>, long>
findBestPath(std::map<std::string, Valve> &Valves,
             std::map<std::string, std::map<std::string, int>> &TravelDistances,
             int ReleaseRate, std::string CurrentValve, int ReleaseValue,
             int Time);
long partTwo(std::map<std::string, Valve> &Valves);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>\n";
        return 1;
    }

    std::map<std::string, Valve> Valves = readInput(argv[1]);

    std::cout << "Part 1 (without elephant): " << partOne(Valves) << "\n";
    std::cout << "Part 2 (with elephant): " << partTwo(Valves) << "\n";

    return 0;
}

std::map<std::string, Valve> readInput(std::string Path) {
    std::map<std::string, Valve> Valves;

    std::ifstream InputFile(Path);
    if (!InputFile.good()) {
        throw std::runtime_error("Failed to open " + Path + " for reading.");
    }

    std::regex LineParser("Valve ([A-Z][A-Z]) has flow rate=([0-9]+); tunnels? "
                          "leads? to valves? (.*)");
    std::smatch LineRegexResults;
    std::string InputLine;
    while (std::getline(InputFile, InputLine)) {
        if (!std::regex_match(InputLine, LineRegexResults, LineParser)) {
            throw std::runtime_error("Failed to parse line \"" + InputLine +
                                     "\"\n");
        }
        std::string ValveName = LineRegexResults[1];
        int Rate = std::stoi(LineRegexResults[2]);

        std::string TunnelString = LineRegexResults[3];
        std::set<std::string> Tunnels;
        size_t pos;
        while ((pos = TunnelString.find(", ")) != std::string::npos) {
            Tunnels.insert(TunnelString.substr(0, pos));
            TunnelString = TunnelString.substr(pos + 2);
        }
        Tunnels.insert(TunnelString);

        Valves.insert({ValveName, Valve(Tunnels, Rate)});
    }

    return Valves;
}

long partOne(std::map<std::string, Valve> &Valves) {
    // Initialize table
    std::map<std::string, std::map<std::string, int>> TravelDistances;
    for (const auto &[FromValveName, FromValve] : Valves) {
        TravelDistances.insert({FromValveName, std::map<std::string, int>()});
        for (const auto &[ToValveName, ToValve] : Valves) {
            TravelDistances[FromValveName].insert(
                {ToValveName, std::numeric_limits<int>::max()});
        }

        // Initialize edges
        TravelDistances[FromValveName][FromValveName] = 0;
        for (const auto &ToValveName : FromValve.Tunnels) {
            TravelDistances[FromValveName][ToValveName] = 1;
        }
    }

    // Do actual Floyd-Warshall
    for (const auto &[IntermediateName, Intermediate] : Valves) {
        for (const auto &[StartName, Start] : Valves) {
            for (const auto &[EndName, End] : Valves) {
                if (TravelDistances[StartName][IntermediateName] !=
                        std::numeric_limits<int>::max() &&
                    TravelDistances[IntermediateName][EndName] !=
                        std::numeric_limits<int>::max() &&
                    TravelDistances[StartName][EndName] >
                        TravelDistances[StartName][IntermediateName] +
                            TravelDistances[IntermediateName][EndName]) {
                    TravelDistances[StartName][EndName] =
                        TravelDistances[StartName][IntermediateName] +
                        TravelDistances[IntermediateName][EndName];
                }
            }
        }
    }

    // Create new valves list with no broken valves
    std::map<std::string, Valve> WorkingValves;
    for (const auto &[CurrentValveName, CurrentValve] : Valves) {
        if (CurrentValveName == "AA" || CurrentValve.Rate != 0) {
            WorkingValves.insert({CurrentValveName, CurrentValve});
        }
    }

    // Run once with AA open and once with it closed (all other movements
    // immediately open ToValve)

    return findBestPath(WorkingValves, TravelDistances, 0, "AA", 0, 0).second;
}

std::pair<std::vector<std::string>, long>
findBestPath(std::map<std::string, Valve> &Valves,
             std::map<std::string, std::map<std::string, int>> &TravelDistances,
             int ReleaseRate, std::string CurrentValve, int ReleaseValue,
             int Time) {
    if (Time > 30)
        return {{CurrentValve}, ReleaseValue};

    std::vector<std::string> BestPath;

    long MaxReleased = 0;

    // Test travel to all possible valves
    for (auto &[NextValveName, NextValve] : Valves) {
        if (NextValve.Open ||
            TravelDistances[CurrentValve][NextValveName] > 30 - Time - 1)
            continue;

        NextValve.Open = true;
        auto [PossiblePath, Released] = findBestPath(
            Valves, TravelDistances, ReleaseRate + NextValve.Rate,
            NextValveName,
            ReleaseValue +
                ReleaseRate *
                    (TravelDistances[CurrentValve][NextValveName] + 1),
            Time + TravelDistances[CurrentValve][NextValveName] + 1);
        if (Released > MaxReleased) {
            MaxReleased = Released;
            BestPath = PossiblePath;
        }
        NextValve.Open = false;
    }

    // Test if just sit down and do nothing
    long Released = ReleaseValue + ReleaseRate * (30 - Time);
    if (Released > MaxReleased) {
        MaxReleased = Released;
    }

    BestPath.insert(BestPath.begin(), CurrentValve);
    return {BestPath, MaxReleased};
}

long partTwo(std::map<std::string, Valve> &Valves) {
    // Initialize table
    std::map<std::string, std::map<std::string, int>> TravelDistances;
    for (const auto &[FromValveName, FromValve] : Valves) {
        TravelDistances.insert({FromValveName, std::map<std::string, int>()});
        for (const auto &[ToValveName, ToValve] : Valves) {
            TravelDistances[FromValveName].insert(
                {ToValveName, std::numeric_limits<int>::max()});
        }

        // Initialize edges
        TravelDistances[FromValveName][FromValveName] = 0;
        for (const auto &ToValveName : FromValve.Tunnels) {
            TravelDistances[FromValveName][ToValveName] = 1;
        }
    }

    // Do actual Floyd-Warshall
    for (const auto &[IntermediateName, Intermediate] : Valves) {
        for (const auto &[StartName, Start] : Valves) {
            for (const auto &[EndName, End] : Valves) {
                if (TravelDistances[StartName][IntermediateName] !=
                        std::numeric_limits<int>::max() &&
                    TravelDistances[IntermediateName][EndName] !=
                        std::numeric_limits<int>::max() &&
                    TravelDistances[StartName][EndName] >
                        TravelDistances[StartName][IntermediateName] +
                            TravelDistances[IntermediateName][EndName]) {
                    TravelDistances[StartName][EndName] =
                        TravelDistances[StartName][IntermediateName] +
                        TravelDistances[IntermediateName][EndName];
                }
            }
        }
    }

    // Create new valves list with no broken valves
    std::map<std::string, Valve> WorkingValves;
    for (const auto &[CurrentValveName, CurrentValve] : Valves) {
        if (CurrentValveName == "AA" || CurrentValve.Rate != 0) {
            WorkingValves.insert({CurrentValveName, CurrentValve});
        }
    }

    const auto [PersonPath, PersonReleased] =
        findBestPath(WorkingValves, TravelDistances, 0, "AA", 0, 4);

    // Remove all locations the person visited from valves
    for (const auto &PersonValve : PersonPath) {
        if (PersonValve != "AA") {
            WorkingValves.extract(PersonValve);
        }
    }

    const auto [ElephantPath, ElephantReleased] =
        findBestPath(WorkingValves, TravelDistances, 0, "AA", 0, 4);

    return PersonReleased + ElephantReleased;
}
