#include <eigen3/Eigen/Dense>
#include <iostream>
#include <regex>
#include <sstream>
#include <stdexcept>

#include "../utility/helpers.hpp"

using NumType = long double;
using NumPair = std::pair<NumType, NumType>;

bool isNearInt(Eigen::Vector2d &v) {
  return (std::abs(v[0] - std::round(v[0])) <= 1e-3 ||
          std::abs(v[1] - std::round(v[1])) <= 1e-3);
}

const short WEIGHT_A = 3;
const short WEIGHT_B = 1;

struct Machine {
  NumPair buttonA;
  NumPair buttonB;
  NumPair prize;

  NumType lowestPrice() const {
    // System of equations:
    // A.first*X + B.first*Y = prize.first
    // Solve with linear algebra
    // AX=B => X = A-1B

    Eigen::Matrix2d A;
    A << buttonA.first, buttonB.first, buttonA.second, buttonB.second;

    if (A.determinant() == 0) {
      return 0;
    }

    Eigen::Vector2d B;
    B << prize.first, prize.second;

    Eigen::Vector2d X = A.inverse() * B;

    return isNearInt(X)
               ? std::round(X[0]) * WEIGHT_A + std::round(X[1]) * WEIGHT_B
               : 0;
  }

  // Debugging purposes
  friend std::ostream &operator<<(std::ostream &os, const Machine &m) {
    std::stringstream ss;
    // ss.imbue(std::locale(""));
    ss << "Machine {\n";
    ss << "\tbuttonA: {" << m.buttonA.first << ", " << m.buttonA.second
       << "}\n";
    ss << "\tbuttonB: {" << m.buttonB.first << ", " << m.buttonB.second
       << "}\n";
    ss << "\tprize: {" << m.prize.first << ", " << m.prize.second << "}\n";
    ss << '}';

    os << ss.str();

    return os;
  }
};

NumPair getFromRegex(const std::string &line, const std::regex &regex) {

  std::smatch match;
  double x = 0.;
  double y = 0.;

  if (std::regex_search(line, match, regex)) {
    x = std::stod(match[1].str());
    y = std::stod(match[2].str());
  }

  return {x, y};
}

int main() {
  const std::regex buttonRe(R"(Button [AB]: X\+(\d+), Y\+(\d+))");
  const std::regex prizeRe(R"(Prize: X=(\d+), Y=(\d+))");

  std::vector<std::string> lines = extractInputLines();

  std::vector<Machine> machines;
  for (size_t i{}; i + 2 < lines.size(); i += 4) {
    Machine machine;

    // Process Button A
    machine.buttonA = getFromRegex(lines[i], buttonRe);

    // Process Button B
    machine.buttonB = getFromRegex(lines[i + 1], buttonRe);

    // Process Prize
    machine.prize = getFromRegex(lines[i + 2], prizeRe);

    // Make sure all of them were read correctly
    for (const auto &pair : {machine.buttonA, machine.buttonB, machine.prize}) {
      const NumPair invalidPair{0, 0};
      if (pair == invalidPair) {
        throw std::runtime_error("[x,y] == {0,0}");
      }
    }

    machines.push_back(machine);
  }

  NumType sum1{};
  NumType sum2{};
  for (auto machine : machines) {
    Machine machine2 = {.buttonA = machine.buttonA,
                        .buttonB = machine.buttonB,
                        .prize = {machine.prize.first + 10'000'000'000'000,
                                  machine.prize.second + 10'000'000'000'000}};
#ifdef DEBUG
    std::cout << machine << std::endl;
    std::cout << machine2 << std::endl;
#endif
    sum1 += machine.lowestPrice();
    sum2 += machine2.lowestPrice();
  }
  std::cout << "Sum: " << sum1 << std::endl;

#ifdef PART2
  // part2 not working
  std::cout << "Sum2: " << sum2 << std::endl;
#endif

  return 0;
}
