import re


def main():
    pattern = r"mul\((\d{1,3}),(\d{1,3})\)"

    sum = 0
    for line in open("input.txt", "r"):
        matches = re.finditer(pattern, line)
        for match in matches:
            a = match[1]
            b = match[2]
            sum += int(a) * int(b)

    print(f"Sum : {sum}")


if __name__ == "__main__":
    main()
