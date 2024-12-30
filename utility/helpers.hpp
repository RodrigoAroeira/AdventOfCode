#pragma once

#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

inline std::string getRawText(const std::string &filename) {

  std::ifstream f(filename);
  if (!f.is_open())
    throw std::runtime_error("Error opening file");

  std::stringstream buffer;
  buffer << f.rdbuf();
  return buffer.str();
}

inline std::vector<std::string> extractLines(const std::string &text) {
  std::istringstream iss(text);
  std::vector<std::string> lines;
  std::string line;
  while (std::getline(iss, line))
    lines.push_back(line);

  return lines;
  // return {std::istream_iterator<std::string>(iss),
  //         std::istream_iterator<std::string>()};
}

inline std::vector<std::string> extractInputLines() {
  std::vector<std::string> lines;
  std::string line;

  while (std::getline(std::cin, line))
    lines.push_back(line);

  return lines;
}
