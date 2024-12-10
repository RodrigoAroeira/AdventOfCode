// https://adventofcode.com/2024/day/10

#include <fstream>
#include <iostream>
#include <iterator>
#include <set>
#include <sstream>
#include <vector>

using CoordinateType = std::pair<int, int>;

std::vector<CoordinateType>
findTrailheadsPos(const std::vector<std::string> &topographyMap) {
  std::vector<CoordinateType> trailheads;
  for (int i{}; i < topographyMap.size(); i++)
    for (int j{}; j < topographyMap[i].size(); j++)
      trailheads.push_back({i, j});

  return trailheads;
}

bool inBounds(const std::vector<std::string> &topographyMap, int x, int y) {
  return x >= 0 && y >= 0 && x < topographyMap.size() &&
         y < topographyMap[x].size();
}

int searchHelper(const std::vector<std::string> &topographyMap, int x, int y,
                 int expectedValue, std::set<CoordinateType> *seen) {

  if (!inBounds(topographyMap, x, y))
    return 0;

  int currentValue = topographyMap[x][y] - '0';

  if (currentValue != expectedValue)
    return 0;

  if (seen && seen->count({x, y}))
    return 0;

  if (expectedValue == 9) {
    if (seen)
      seen->insert({x, y});
    return 1;
  }

  int nextValue = expectedValue + 1;

  int right = searchHelper(topographyMap, x + 1, y, nextValue, seen);
  int left = searchHelper(topographyMap, x - 1, y, nextValue, seen);
  int down = searchHelper(topographyMap, x, y + 1, nextValue, seen);
  int up = searchHelper(topographyMap, x, y - 1, nextValue, seen);

  int totalPaths = right + up + left + down;

  return totalPaths;
}

int searchTrail(const std::vector<std::string> &topographyMap, int x, int y,
                int expectedValue, std::set<CoordinateType> &seen) {

  return searchHelper(topographyMap, x, y, expectedValue, &seen);
}

int searchAllTrails(const std::vector<std::string> &topographyMap, int x, int y,
                    int expectedValue) {

  return searchHelper(topographyMap, x, y, expectedValue, nullptr);
}

std::string getRawText(const std::string &filename) {

  std::ifstream f(filename);
  if (!f.is_open())
    throw std::runtime_error("Error opening file");

  std::stringstream buffer;
  buffer << f.rdbuf();
  return buffer.str();
}

std::vector<std::string> extractLines(const std::string &text) {
  std::istringstream iss(text);
  return {std::istream_iterator<std::string>(iss),
          std::istream_iterator<std::string>()};
}

int main(int argc, char **argv) {

  const char *filename = argc > 1 ? argv[1] : "input.txt";

  std::string fileContent = getRawText(filename);

  std::vector<std::string> topographyMap = extractLines(fileContent);

  std::vector<CoordinateType> trailheads = findTrailheadsPos(topographyMap);

  int partialTrailCount{}; // Didn't know how else to call, because I did part2
                           // first somehow
  int totalTrailCount{};

  for (auto &startingPos : trailheads) {
    std::set<CoordinateType> seen;
    int partialTrail = searchTrail(topographyMap, startingPos.first,
                                   startingPos.second, 0, seen);

    int fullTrails = searchAllTrails(topographyMap, startingPos.first,
                                     startingPos.second, 0);

    partialTrailCount += partialTrail;
    totalTrailCount += fullTrails;
  }

  std::cout << partialTrailCount << std::endl;
  std::cout << totalTrailCount << std::endl;
}
