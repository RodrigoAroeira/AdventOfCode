import sys


def main() -> None:
    filename = sys.argv[1] if sys.argv[1:] else "input.txt"

    with open(filename, "r") as f:
        txt = f.read().strip()
        lines = txt.splitlines()

    left, right = [], []

    for line in lines:
        n1, n2 = map(int, line.split())
        left.append(n1)
        right.append(n2)

    left.sort()
    right.sort()

    sum1 = 0
    for i in range(len(left)):
        dist = abs(left[i] - right[i])

        sum1 += dist

    print(f"Part 1: {sum1} ")

    appears_on_right = {n: sum(1 for r in right if r == n) for n in left}

    sum2 = 0
    for n in left:
        sum2 += n * appears_on_right[n]

    print(f"Part 2: {sum2} ")


if __name__ == "__main__":
    main()
