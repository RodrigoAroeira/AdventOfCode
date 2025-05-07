// https://adventofcode.com/2024/day/7

#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "../utility/helpers.hpp"

using IntType = long long;

IntType evalExpr(const std::vector<IntType> &nums,
                 const std::vector<std::string> &ops) {
  IntType result{};
  for (size_t i{1}; i < nums.size(); ++i) {
    auto op = ops[i - 1];
    auto num = nums[i];

    if (op == "+")
      result += num;
    else if (op == "*")
      result *= num;
    else if (op == "||") {
      const std::string concat = std::to_string(result) + std::to_string(num);
      std::istringstream ss(concat);
      ss >> result;
    }
  }
  return result;
}

std::vector<std::vector<std::string>>
generateOps(const std::vector<std::string> &ops, size_t length) {
  if (length == 0) {
    return {{}};
  }

  std::vector<std::vector<std::string>> result;
  for (const auto &op : ops) {
    for (auto &smaller : generateOps(ops, length - 1)) {
      smaller.push_back(op);
      result.push_back(smaller);
    }
  }

  return result;
}

int main(int argc, const char **argv) {
  std::vector<std::string> ops1 = {"+", "*"};
  std::vector<std::string> ops2 = ops1;
  ops2.push_back("||");

  const std::string filename = argc > 1 ? argv[1] : "input.in";

  auto txt = getRawText(filename);

  IntType res{};
  IntType res2{};

  for (auto &line : extractLines(txt)) {
    std::vector<IntType> nums;
    IntType expected;
    {
      std::istringstream iss(line);
      std::string part;
      std::getline(iss, part, ':');
      expected = std::stoll(part);

      while (iss >> part) {
        nums.push_back(std::stoll(part));
      }
    }

    for (auto &ops : generateOps(ops1, nums.size())) {
      if (evalExpr(nums, ops) == expected) {
        res += expected;
        break;
      }
    }
    for (auto &ops : generateOps(ops2, nums.size())) {
      if (evalExpr(nums, ops) == expected) {
        res2 += expected;
        break;
      }
    }
  }

  std::cout << res << std::endl;
  std::cout << res2 << std::endl;
}
