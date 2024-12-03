// https://adventofcode.com/2024/day/3

#include <fstream>
#include <iostream>
#include <regex>

int main() {
  using namespace std;
  std::ifstream inputFile("input.txt");
  if (!inputFile.is_open()) {
    std::cerr << "Error opening file!" << std::endl;
    return -1;
  }

  std::string input;
  {
    std::string s;
    while (inputFile >> s)
      input.append(s);
  }

  const regex p(R"(mul\((\d{1,3}),(\d{1,3})\))");
  sregex_iterator begin(input.begin(), input.end(), p);
  sregex_iterator end;

  int sum = 0;

  for (sregex_iterator it(input.begin(), input.end(), p); it != end; it++) {
    std::smatch match = *it;
    std::string group1 = match[1].str();
    std::string group2 = match[2].str();

    int a = std::stoi(group1);
    int b = std::stoi(group2);

    sum += a * b;
  }
  std::cout << "Sum : " << sum << std::endl;
}
