// https://adventofcode.com/2024/day/5

#include <fstream>
#include <iostream>
#include <iterator>
#include <sstream>
#include <unordered_map>
#include <vector>

std::vector<std::string> extractLines(const std::string &s) {
  std::istringstream iss(s);
  return {std::istream_iterator<std::string>(iss),
          std::istream_iterator<std::string>()};
}

bool isUpdateValid(
    const std::vector<int> &order,
    const std::unordered_map<int, std::vector<int>> &priorityRules) {

  std::unordered_map<int, int> position;

  for (size_t i{}; i < order.size(); i++) {
    int page = order[i];
    position[page] = i;
  }

  for (const auto &[page, dependencies] : priorityRules) {
    if (position.find(page) == position.end())
      continue;

    for (const int &dependency : dependencies) {
      if (position.find(dependency) != position.end() &&
          position[page] > position[dependency]) {
        return false;
      }
    }
  }

  return true;
}

int main() {
  std::string rules, updates;

  {
    std::stringstream buffer;
    buffer << std::ifstream("input.txt").rdbuf();

    std::string buffStr = buffer.str();
    size_t sectionPos = buffStr.find("\n\n");

    rules = buffStr.substr(0, sectionPos);
    updates = buffStr.substr(sectionPos + 2);
  }

  std::unordered_map<int, std::vector<int>> priorityRules;
  for (const std::string &line : extractLines(rules)) {
    std::stringstream ss(line); // X | Y
    int x, y;
    char _;
    ss >> x >> _ >> y;
    priorityRules[x].push_back(y);
  }

  int sum{};
  for (const std::string &line : extractLines(updates)) {
    std::istringstream iss(line);
    std::string numStr;
    std::vector<int> order;

    while (std::getline(iss, numStr, ',')) {
      order.push_back(std::stoi(numStr));
    }

    if (!isUpdateValid(order, priorityRules) || order.empty())
      continue;

    int mid = order.size() / 2; // always odd
    sum += order[mid];
  }
  std::cout << sum << std::endl;
}
