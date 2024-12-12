// https://adventofcode.com/2024/day/11

#include <array>
#include <cmath>
#include <iostream>
#include <vector>

#include "../utility/helpers.hpp"

using IntType = unsigned long long;

const IntType TOTAL_BLINKS = 25;

void printVector(const std::vector<IntType> &vec) {
  for (const auto &n : vec)
    std::cout << n << ' ';
  std::cout << std::endl;
}

[[nodiscard]] int getNumberOfDigits(IntType n) {
  return std::floor(std::log10(n)) + 1;
}

[[nodiscard]] IntType ZeroRule(IntType n) { return 1; }

[[nodiscard]] std::array<IntType, 2> EvenDigitsRule(IntType n) {
  std::string asStr = std::to_string(n);
  int divisionPoint = asStr.size() / 2;

  IntType half1 = std::stoull(asStr.substr(0, divisionPoint));
  IntType half2 = std::stoull(asStr.substr(divisionPoint));

  return {half1, half2};
}

[[nodiscard]] IntType ElseRule(IntType n) { return n * 2024; }

void applyRules(std::vector<IntType> &nums) {

  std::vector<IntType> newNums;

  for (int i{}; i < nums.size(); i++) {
    IntType num = nums[i];

    if (num == 0) {

      newNums.push_back(ZeroRule(num));

    } else if (getNumberOfDigits(num) % 2 == 0) {

      auto [half1, half2] = EvenDigitsRule(num);

      newNums.push_back(half1);
      newNums.push_back(half2);

    } else {
      newNums.push_back(ElseRule(num));
    }
  }
  nums = std::move(newNums);
}

int main() {
  std::string txt = getRawText("input.txt");
  std::vector<IntType> stones;

  {
    std::istringstream iss(txt);
    IntType n;
    while (iss >> n)
      stones.push_back(n);
  }

  std::cout << "Initial stones: ";
  printVector(stones);

  for (int i{1}; i <= TOTAL_BLINKS; i++) {
    applyRules(stones);
  }

  std::cout << stones.size() << std::endl;
}
