#include <algorithm>
#include <fstream>
#include <iostream>
#include <memory>
#include <sstream>
#include <stack>
#include <vector>

// I went about this the entire wrong way, but my solution still works so it's
// fiiiiiiiinnnnneeeee.
// Just don't give it a larger input than the puzzle LOL.

class Command {
  public:
    enum class Type { Cd, Ls };

  private:
    Type type_;
    std::string arg_;
    std::vector<std::string> output_;

  public:
    Command(Type type, std::string arg, std::vector<std::string> output);

    const Type &type() const { return type_; }
    const std::string &arg() const { return arg_; }
    const std::vector<std::string> &output() const { return output_; }
};

class File {
  private:
    std::string name_;
    long size_;

  public:
    File(std::string &name, long size);

    const std::string &name() const { return name_; };
    long size() const { return size_; };
};

class Directory {
  private:
    std::string name_;
    std::vector<std::shared_ptr<Directory>> dirs_;
    std::vector<std::shared_ptr<File>> files_;

  public:
    Directory(const std::string &name);

    const std::string &name() const { return name_; };
    long size() const;
    long sizeLt(long maxSize) const;
    std::vector<long> nonSumSizeGt(long minSize) const;
    bool hasChild(const std::string &name) const;
    void addChildDir(std::shared_ptr<Directory> child);
    void addChildFile(std::shared_ptr<File> child);
    std::shared_ptr<Directory> getChildDir(const std::string &name);
    std::shared_ptr<File> getChildFile(const std::string &name);
};

const std::vector<Command> parseCommands(const std::string &fname);

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "c'mon\n";
        return 1;
    }

    std::vector<Command> commands = parseCommands(argv[1]);
    std::cout << "Commands parsed\n";

    auto fs = std::make_shared<Directory>("/");
    std::stack<std::shared_ptr<Directory>> fsTraveller;
    fsTraveller.push(fs);
    for (auto &command : commands) {
        switch (command.type()) {
        case Command::Type::Cd: {
            if (command.arg() == "..") {
                fsTraveller.pop();
                break;
            }
            if (!fsTraveller.top()->hasChild(command.arg()))
                throw std::runtime_error("cd: no such file or directory");
            auto childObj = fsTraveller.top()->getChildDir(command.arg());
            auto child = std::dynamic_pointer_cast<Directory>(childObj);
            fsTraveller.push(child);
            break;
        }

        case Command::Type::Ls: {
            auto self = fsTraveller.top();
            for (std::string newChild : command.output()) {
                std::stringstream ss(newChild);
                std::string name;
                long size;
                try {
                    size = std::stol(newChild);
                    std::getline(ss, name, ' ');
                    std::getline(ss, name);
                    self->addChildFile(std::make_shared<File>(name, size));
                } catch (std::exception &e) {
                    std::getline(ss, name, ' ');
                    std::getline(ss, name);
                    self->addChildDir(std::make_shared<Directory>(name));
                }
            }
            break;
        }
        }
    }

    std::cout << "Size: " << fs->sizeLt(100000) << "\n";

    long totalSize = fs->size();
    long needed = 30000000 - (70000000 - totalSize);
    std::cout << "Size for part 2: " << fs->nonSumSizeGt(needed)[0] << "\n";

    return 0;
}

Command::Command(Type type, std::string arg, std::vector<std::string> output)
    : type_(type), arg_(arg), output_(output) {}

Directory::Directory(const std::string &name) : name_(name) {
    dirs_.clear();
    files_.clear();
}

long Directory::size() const {
    long total = 0;

    for (auto child : dirs_)
        total += child->size();
    for (auto child : files_)
        total += child->size();

    return total;
}

File::File(std::string &name, long size) : name_(name), size_(size) {}

const std::vector<Command> parseCommands(const std::string &fname) {
    std::vector<Command> commands;
    std::vector<std::string> input;

    std::ifstream inputFile(fname);
    if (!inputFile.is_open())
        throw std::runtime_error("Failed to open for reading.");

    std::string inputLine;
    while (std::getline(inputFile, inputLine))
        input.push_back(inputLine);

    auto line = input.begin() + 1;
    while (line != input.end()) {
        Command::Type cmdType;
        std::string cmdArg;
        std::vector<std::string> cmdOutput;
        if (line->starts_with("$ cd")) {
            cmdType = Command::Type::Cd;
            std::stringstream ss(*line);
            std::getline(ss, cmdArg, ' ');
            std::getline(ss, cmdArg, ' ');
            std::getline(ss, cmdArg);
        } else if (line->starts_with("$ ls")) {
            cmdType = Command::Type::Ls;
            for (++line; line != input.end() && !line->starts_with("$ ");
                 ++line)
                cmdOutput.push_back(*line);
            --line;
        } else {
            throw std::runtime_error(
                std::string("Failed to read data: line \"") + *line +
                "\" invalid.");
        }

        commands.push_back(Command(cmdType, cmdArg, cmdOutput));

        ++line;
    }

    return commands;
}

bool Directory::hasChild(const std::string &name) const {
    for (auto child : dirs_) {
        if (child->name() == name)
            return true;
    }
    for (auto child : files_) {
        if (child->name() == name)
            return true;
    }
    return false;
};

void Directory::addChildDir(std::shared_ptr<Directory> child) {
    if (hasChild(child->name()))
        return;

    dirs_.push_back(child);
};

void Directory::addChildFile(std::shared_ptr<File> child) {
    if (hasChild(child->name()))
        return;

    files_.push_back(child);
}

std::shared_ptr<Directory> Directory::getChildDir(const std::string &name) {
    for (auto child : dirs_) {
        if (child->name() == name)
            return child;
    }

    return nullptr;
};

std::shared_ptr<File> Directory::getChildFile(const std::string &name) {
    for (auto child : files_) {
        if (child->name() == name)
            return child;
    }

    return nullptr;
}

long Directory::sizeLt(long maxSize) const {
    long selfSize = size();

    long total = (selfSize <= maxSize) ? selfSize : 0;
    for (auto child : dirs_)
        total += child->sizeLt(maxSize);

    return total;
}

std::vector<long> Directory::nonSumSizeGt(long minSize) const {
    std::vector<long> fileSizes;

    long selfSize = size();
    if (selfSize > minSize)
        fileSizes.push_back(selfSize);

    for (auto child : dirs_) {
        auto childSizes = child->nonSumSizeGt(minSize);
        fileSizes.insert(fileSizes.end(), childSizes.begin(), childSizes.end());
    }

    std::sort(fileSizes.begin(), fileSizes.end());
    return fileSizes;
};
