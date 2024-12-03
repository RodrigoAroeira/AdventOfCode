#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <type_traits>
#include <vector>

const int MIN_SAFE_DIST = 1;
const int MAX_SAFE_DIST = 3;

template <typename T>
typename std::enable_if<std::is_arithmetic<T>::value, bool>::type
isMonotonic(const std::vector<T> &nums) {
  bool increasing = true;
  bool decreasing = true;

  for (size_t i = 0; i < nums.size() - 1; i++) {
    if (nums[i + 1] > nums[i])
      decreasing = false;
    if (nums[i + 1] < nums[i])
      increasing = false;
    if (!increasing && !decreasing)
      break;
  }
  return increasing || decreasing;
}

bool isSafe(const std::vector<int> &nums) {
  if (!isMonotonic(nums))
    return false;

  for (size_t i = 0; i < nums.size() - 1; i++) {
    size_t diff = abs(nums[i] - nums[i + 1]);

    if (diff < MIN_SAFE_DIST || diff > MAX_SAFE_DIST)
      return false;
  }

  return true;
}

bool isSafeDampened(const std::vector<int> &nums) {
  if (isSafe(nums))
    return true;

  for (size_t i = 0; i < nums.size(); i++) {
    std::vector<int> other = nums;
    other.erase(other.begin() + i);

    if (isSafe(other)) {
      return true;
    }
  }
  return false;
}

int main() {

  std::ifstream inputFile("input.txt");
  std::string line;

  int safes = 0;
  int safes2 = 0;
  while (std::getline(inputFile, line)) {
    std::stringstream ss(line);
    std::vector<int> nums;
    int num;
    while (ss >> num)
      nums.push_back(num);

    if (isSafe(nums))
      safes++;

    if (isSafeDampened(nums))
      safes2++;
  }

  std::cout << "Safes: " << safes << std::endl;
  std::cout << "Safes, dampened: " << safes2 << std::endl;
}
