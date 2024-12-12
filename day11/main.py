from functools import cache
from math import floor, log10


def get_number_of_digits(n: int) -> int:
    return floor(log10(n)) + 1


def separate_number(n: int) -> tuple[int, int]:
    as_str = str(n)

    half1 = as_str[: len(as_str) // 2]
    half2 = as_str[len(as_str) // 2 :]

    return (int(half1), int(half2))


@cache
def count(stone: int, steps: int):
    if steps == 0:
        return 1
    if stone == 0:
        return count(1, steps - 1)

    if get_number_of_digits(stone) % 2 == 0:
        half1, half2 = separate_number(stone)
        return count(half1, steps - 1) + count(half2, steps - 1)

    return count(stone * 2024, steps - 1)


def main():
    with open("input.txt", "r") as file:
        txt = file.read().split()
        stones = [int(i) for i in txt]

    for step in (25, 75):
        print(sum(count(stone, step) for stone in stones))


if __name__ == "__main__":
    main()
