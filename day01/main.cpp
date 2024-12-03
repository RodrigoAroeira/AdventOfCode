// https://adventofcode.com/2024/day/1

#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <vector>

int main() {
  // part 1
  std::vector<int> left;
  std::vector<int> right;
  std::vector<int> distances;
  std::ifstream inputFile("input.txt");
  std::string line;

  // line contains two numbers separated by a space
  while (std::getline(inputFile, line)) {
    std::istringstream iss(line);
    int n1, n2;
    iss >> n1 >> n2;
    left.push_back(n1);
    right.push_back(n2);
  }

  std::sort(left.begin(), left.end());
  std::sort(right.begin(), right.end());

  int sum = 0;
  for (int i = 0; i < left.size(); i++) {
    int distance = left[i] - right[i];
    if (distance < 0) {
      distance = -distance;
    }
    sum += distance;
  }

  std::cout << "Part 1: " << sum << std::endl;

  // Part 2
  std::map<int, int> appearsOnRight;

  for (const int l : left) {
    int count = 0;
    for (const int r : right) {
      if (r == l)
        count++;
    }
    appearsOnRight[l] = count;
  }

  int sum2 = 0;
  for (const int l : left) {
    sum2 += l * appearsOnRight[l];
  }

  std::cout << "Part 2: " << sum2 << std::endl;

  return 0;
}
