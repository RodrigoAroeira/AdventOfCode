// https://adventofcode.com/2024/day/4

#include <algorithm>
#include <iostream>
#include <vector>

#include "../utility/helpers.hpp"

using StrVec = std::vector<std::string>;

StrVec transpose_matrix(StrVec &lines) {

  StrVec transposed(lines[0].size());

  for (auto &line : lines) {
    for (int i{}; char c : line) {
      transposed[i++].push_back(c);
    }
  }

  return transposed;
}

StrVec getDiagonalsLR(StrVec &lines) {
  StrVec diagonals;
  size_t rows = lines.size();

  for (size_t start{}; start < rows; ++start) {
    std::string diagonal;
    size_t row = start, col = 0;

    while (row < rows && col < lines[row].size()) {
      diagonal += lines[row][col];
      ++row;
      ++col;
    }
    diagonals.push_back(diagonal);
  }

  size_t max = std::max_element(lines.begin(), lines.end(),
                                [](const std::string &a, const std::string &b) {
                                  return a.size() < b.size();
                                })
                   ->size();

  for (size_t start{}; start < max; ++start) {
    std::string diagonal;
    size_t row = 0, col = start;

    while (row < rows && col < lines[row].size()) {
      diagonal += lines[row][col];
      ++row;
      ++col;
    }

    diagonals.push_back(diagonal);
  }

  return diagonals;
}

StrVec getDiagonalsRL(StrVec &lines) {
  size_t rows = lines.size();
  StrVec diagonals;

  for (size_t start{}; start < rows; ++start) {
    std::string diagonal;
    size_t row = start, col = lines[start].size() - 1;

    while (row < rows && col >= 0) {
      diagonal += lines[row][col];
      ++row;
      --col;
    }

    diagonals.push_back(diagonal);
  }

  int max = std::max_element(lines.begin(), lines.end(),
                             [](const std::string &a, const std::string &b) {
                               return a.size() < b.size();
                             })
                ->size();

  for (int start{max - 2}; start < -1; --start) {
    std::string diagonal;
    size_t row = 0, col = start;

    while (row < rows && col >= 0 && col < lines[row].size()) {
      diagonal += lines[row][col];
      ++row;
      --col;
    }

    diagonals.push_back(diagonal);
  }

  return diagonals;
}

size_t findXMASbyLine(const std::string &line) {
  size_t count{};
  size_t pos = line.find("XMAS");

  while (pos != std::string::npos) {
    ++count;
    pos = line.find("XMAS", pos + 4);
  }

  return count;
}

size_t findXMASbyLineAndReverse(const std::string &line) {
  size_t counter{};
  std::string reversed(line.rbegin(), line.rend());

  counter += findXMASbyLine(line);
  counter += findXMASbyLine(reversed);

  return counter;
}

size_t findXMASinLines(const StrVec &lines) {
  size_t counter{};

  for (const auto &line : lines) {
    counter += findXMASbyLineAndReverse(line);
  }

  return counter;
}

bool isCenterOf3x3(const std::tuple<size_t, size_t> &pos,
                   const StrVec &fileLines) {
  auto [x, y] = pos;
  if (x - 1 < 0 || x + 1 >= fileLines.size() || y - 1 < 0 ||
      y + 1 >= fileLines[0].size())
    return false;

  return true;
}

int main() {

  auto lines = extractLines(getRawText("input.txt"));

  auto transposed = transpose_matrix(lines);

  auto LRDiag = getDiagonalsLR(lines);
  auto RLDiag = getDiagonalsRL(lines);

  size_t counter{};
  counter += findXMASinLines(lines);
  counter += findXMASinLines(transposed);
  counter += findXMASinLines(LRDiag);
  counter += findXMASinLines(RLDiag);

  std::cout << counter << std::endl;
}
