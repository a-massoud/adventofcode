#include <array>
#include <iomanip>
#include <iostream>
#include <limits>
#include <openssl/md5.h>

// This one is hacky as all hell and uses a deprecated OpenSSL API, but I didn't
// want to try and figure out the whole contexts thing.

bool hasLeadingZeroes(
    const std::array<unsigned char, MD5_DIGEST_LENGTH> &buffer, size_t n) {
    size_t j = n / 2;
    for (size_t i = 0; i < j; ++i) {
        if (buffer[i] != 0)
            return false;
    }
    if (n % 2 && buffer[n / 2] > 0xF)
        return false;
    return true;
}

unsigned long findHashWithZeroes(const std::string &input, size_t n) {
    unsigned long i;
    std::array<unsigned char, MD5_DIGEST_LENGTH> md5Buffer;
    for (i = 0; i < std::numeric_limits<unsigned long>::max(); ++i) {
        std::string key = input + std::to_string(i);
        MD5((unsigned char *)(key.c_str()), key.length(), md5Buffer.data());
        if (hasLeadingZeroes(md5Buffer, n))
            break;
    }
    return i;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <puzzle input>\n";
        return 0;
    }

    std::cout << "Part 1 results: " << findHashWithZeroes(argv[1], 5) << "\n";
    std::cout << "Part 2 results: " << findHashWithZeroes(argv[1], 6) << "\n";

    return 0;
}
