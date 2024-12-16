// https://adventofcode.com/2024/day/11

#include <array>
#include <cmath>
#include <iostream>
#include <map>
#include <vector>

#include "../utility/helpers.hpp"

using IntType = unsigned long long;

[[nodiscard]] int getNumberOfDigits(IntType n) {
  return n == 0 ? 1 : std::floor(std::log10(n)) + 1;
}

IntType stringToInt(const std::string &str) {
  std::istringstream iss(str);
  IntType val;
  iss >> val;
  return val;
}

[[nodiscard]] std::array<IntType, 2> EvenDigitsRule(IntType n) {
  std::string asStr = std::to_string(n);
  int divisionPoint = asStr.size() / 2;

  IntType half1 = stringToInt(asStr.substr(0, divisionPoint));
  IntType half2 = stringToInt(asStr.substr(divisionPoint));

  return {half1, half2};
}

[[nodiscard]] IntType ElseRule(IntType n) { return n * 2024; }

IntType countCached(IntType stone, IntType steps,
                    std::map<std::pair<IntType, IntType>, IntType> &cache) {
  auto key = std::make_pair(stone, steps);
  if (cache.find(key) != cache.end())
    return cache[key];

  if (steps == 0)
    return 1;
  if (stone == 0)
    return countCached(1, steps - 1, cache);

  IntType res{};

  if (getNumberOfDigits(stone) % 2 == 0) {
    auto [half1, half2] = EvenDigitsRule(stone);
    res = countCached(half1, steps - 1, cache) +
          countCached(half2, steps - 1, cache);
  } else {
    res = countCached(stone * 2024, steps - 1, cache);
  }

  return cache[key] = res;
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

  std::map<std::pair<IntType, IntType>, IntType> cache;
  for (IntType blink : {25, 75}) {
    IntType sum{};
    for (IntType stone : stones) {
      sum += countCached(stone, blink, cache);
    }

    std::cout << sum << '\n';
  }
}
