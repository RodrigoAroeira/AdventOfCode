# https://adventofcode.com/2024/day/7

import sys
from itertools import product


def eval_expr(nums: list[int], ops: tuple[str, ...]) -> int:
    result = nums[0]
    for i, num in enumerate(nums[1:], start=1):
        match ops[i - 1]:
            case "+":
                result += num
            case "*":
                result *= num
            case "||":
                result = int(str(result) + str(num))
            case invalid:
                print(f"There should be no such expression '{invalid}'")
                exit(1)

    return result


def main() -> None:
    operators1 = ["+", "*"]
    operators2 = operators1 + ["||"]
    filename = sys.argv[1] if sys.argv[1:] else "input.in"
    with open(filename, "r") as f:
        lines = [line.strip() for line in f.readlines() if line.strip()]

    res = 0
    res2 = 0
    for line in lines:
        lhs, rhs = line.split(": ")
        expected = int(lhs)
        nums = list(map(int, rhs.split()))

        for ops in product(operators1, repeat=len(nums) - 1):
            if eval_expr(nums, ops) == expected:
                res += expected
                break

        for ops2 in product(operators2, repeat=len(nums) - 1):
            if eval_expr(nums, ops2) == expected:
                res2 += expected
                break

    print(res)
    print(res2)


if __name__ == "__main__":
    main()
