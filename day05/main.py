from collections import defaultdict


def is_correct_update(
    update_order: list[int], priority_rules: dict[int, list[int]]
) -> bool:
    position = {page: idx for idx, page in enumerate(update_order)}

    # Check each rule
    for page, dependencies in priority_rules.items():
        if page not in position:
            continue  # Skip rules for pages not in the update
        for dependency in dependencies:
            if dependency in position and position[page] > position[dependency]:
                return False  # Rule violated: X appears after Y

    return True


def extract_lines(s: str) -> list[str]:
    return [line for line in s.strip().split("\n")]


def main() -> None:
    with open("input.txt") as f:
        txt = f.read().strip()

    rules, updates = txt.split("\n\n")

    priority_rules = defaultdict(list)

    for line in extract_lines(rules):
        x, y = map(int, line.split("|"))
        priority_rules[x].append(y)

    sum = 0
    for line in updates.split("\n"):
        order = list(map(int, line.split(",")))
        if not is_correct_update(order, priority_rules):
            continue

        mid = len(order) // 2  # always odd
        sum += order[mid]

    print(sum)


if __name__ == "__main__":
    main()
