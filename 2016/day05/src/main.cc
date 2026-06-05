// Okay last one

#include <cstddef>
#include <cstdint>
#include <format>
#include <iostream>
#include <memory>
#include <openssl/evp.h>
#include <print>
#include <span>
#include <string>
#include <string_view>
#include <vector>

auto findPasswordOrdered(const std::string_view id) -> std::string {
  constexpr auto freeMDCtx{
      [](EVP_MD_CTX *ctx) -> void { EVP_MD_CTX_free(ctx); }};
  std::unique_ptr<EVP_MD_CTX, decltype(freeMDCtx)> ctx{EVP_MD_CTX_new(),
                                                       freeMDCtx};

  std::string password;
  password.reserve(8);

  std::vector hash(EVP_MD_size(EVP_md5()), static_cast<std::uint8_t>(0));

  int i{0};
  while (password.size() < 8) {
    EVP_DigestInit_ex(ctx.get(), EVP_md5(), nullptr);

    auto tempId{std::format("{}{}", id, i)};

    EVP_DigestUpdate(ctx.get(), tempId.c_str(), tempId.size());

    auto hashSize{static_cast<unsigned int>(hash.size())};
    EVP_DigestFinal_ex(ctx.get(), hash.data(), &hashSize);

    if (hashSize >= 3 && hash[0] == 0 && hash[1] == 0 && hash[2] < 16) {
      if (hash[2] < 10) {
        password.push_back('0' + hash[2]);
      } else {
        password.push_back('a' + hash[2] - 10);
      }
    }

    ++i;
  }

  return password;
}

auto findPasswordUnordered(const std::string_view id) -> std::string {
  constexpr auto freeMDCtx{
      [](EVP_MD_CTX *ctx) -> void { EVP_MD_CTX_free(ctx); }};
  std::unique_ptr<EVP_MD_CTX, decltype(freeMDCtx)> ctx{EVP_MD_CTX_new(),
                                                       freeMDCtx};

  std::string password{"________"};
  int foundCount{0};

  std::vector hash(EVP_MD_size(EVP_md5()), static_cast<std::uint8_t>(0));

  int i{0};
  while (foundCount < 8) {
    EVP_DigestInit_ex(ctx.get(), EVP_md5(), nullptr);

    auto tempId{std::format("{}{}", id, i)};

    EVP_DigestUpdate(ctx.get(), tempId.c_str(), tempId.size());

    auto hashSize{static_cast<unsigned int>(hash.size())};
    EVP_DigestFinal_ex(ctx.get(), hash.data(), &hashSize);

    if (hashSize >= 4 && hash[0] == 0 && hash[1] == 0 && hash[2] < 8 &&
        password[hash[2]] == '_') {
      ++foundCount;
      auto v{hash[3] >> 4};
      if (v < 10) {
        password[hash[2]] = '0' + v;
      } else {
        password[hash[2]] = 'a' + v - 10;
      }
    }

    ++i;
  }

  return password;
}

auto main(int argc, const char *argv[]) -> int {
  const std::span args{argv, static_cast<std::size_t>(argc)};
  if (args.size() < 2) {
    std::println(std::cerr, "no input provided");
    return 1;
  }

  auto orderedPassword{findPasswordOrdered(args[1])};
  std::println("Password (ordered): {}", orderedPassword);

  auto unorderedPassword{findPasswordUnordered(args[1])};
  std::println("Password (unordered): {}", unorderedPassword);

  return 0;
}
