def transposeLinesMatrix(fileLines: list[str]) -> list[str]:
    return ["".join(row) for row in zip(*fileLines)]


def get_diagonals_lr(fileLines: list[str]) -> list[str]:
    rows = len(fileLines)
    diagonals_lr = []

    # Top-left to bottom-right (↘)
    for start_row in range(rows):
        diagonal = []
        row, col = start_row, 0
        while row < rows and col < len(fileLines[row]):
            diagonal.append(fileLines[row][col])
            row += 1
            col += 1
        diagonals_lr.append("".join(diagonal))

    for start_col in range(1, max(len(row) for row in fileLines)):
        diagonal = []
        row, col = 0, start_col
        while row < rows and col < len(fileLines[row]):
            diagonal.append(fileLines[row][col])
            row += 1
            col += 1
        diagonals_lr.append("".join(diagonal))

    return diagonals_lr


def get_diagonals_rl(fileLines: list[str]) -> list[str]:
    rows = len(fileLines)
    diagonals_rl = []

    # Top-right to bottom-left (↙)
    for start_row in range(rows):
        diagonal = []
        row, col = start_row, len(fileLines[start_row]) - 1
        while row < rows and col >= 0:
            diagonal.append(fileLines[row][col])
            row += 1
            col -= 1
        diagonals_rl.append("".join(diagonal))

    for start_col in range(max(len(row) for row in fileLines) - 2, -1, -1):
        diagonal = []
        row, col = 0, start_col
        while row < rows and col >= 0 and col < len(fileLines[row]):
            diagonal.append(fileLines[row][col])
            row += 1
            col -= 1
        diagonals_rl.append("".join(diagonal))

    return diagonals_rl


def find_xmas_by_line(line: str) -> int:
    return line.count("XMAS")


def find_xmas_by_line_and_reverse(line: str) -> int:
    counter = 0
    counter += find_xmas_by_line(line)
    counter += find_xmas_by_line(line[::-1])
    return counter


def find_xmas_in_lines(lines: list[str]) -> int:
    counter = 0
    for line in lines:
        counter += find_xmas_by_line_and_reverse(line)

    return counter


def is_center_of_3x3(pos: tuple[int, int], fileLines: list[str]) -> bool:
    x, y = pos
    if x - 1 < 0 or x + 1 >= len(fileLines) or y - 1 < 0 or y + 1 >= len(fileLines[0]):
        return False
    return True


def print3x3sub(pos: tuple[int, int], matrix: list):
    x, y = pos
    for i in range(-1, 2):
        for j in range(-1, 2):
            print(matrix[x + i][y + j], end=" ")
        print()


def main() -> None:

    with open("input.txt") as f:
        f.seek(0)
        lines = f.readlines()

    transposed = transposeLinesMatrix(lines)

    LR_diagonals = get_diagonals_lr(lines)
    RL_diagonals = get_diagonals_rl(lines)

    counter = 0

    counter += find_xmas_in_lines(lines)
    counter += find_xmas_in_lines(transposed)
    counter += find_xmas_in_lines(LR_diagonals)
    counter += find_xmas_in_lines(RL_diagonals)

    print(counter)

    # part2

    counter2 = 0
    for i in range(len(lines)):
        line = lines[i]
        for j in range(len(line)):
            if not is_center_of_3x3((i, j), lines):
                continue
            if lines[i][j].upper() != "A":
                continue
            diag1 = (lines[i - 1][j - 1] + lines[i][j] + lines[i + 1][j + 1]).upper()
            diag2 = (lines[i - 1][j + 1] + lines[i][j] + lines[i + 1][j - 1]).upper()
            if any(diag == "MAS" for diag in (diag1, diag1[::-1])) and any(
                diag == "MAS" for diag in (diag2, diag2[::-1])
            ):
                counter2 += 1

    print(counter2)


if __name__ == "__main__":
    main()
