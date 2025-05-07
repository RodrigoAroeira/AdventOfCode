MIN_SAFE_DIST = 1
MAX_SAFE_DIST = 3


def is_monotonic(nums: list) -> bool:
    increasing = True
    decreasing = True

    for i in range(len(nums) - 1):
        if nums[i + 1] > nums[i]:
            decreasing = False
        if nums[i + 1] < nums[i]:
            increasing = False
        if not increasing and not decreasing:
            break

    return increasing or decreasing


def is_safe(nums: list) -> bool:
    if not is_monotonic(nums):
        return False

    for i in range(len(nums) - 1):
        diff = abs(nums[i] - nums[i + 1])

        if diff < MIN_SAFE_DIST or diff > MAX_SAFE_DIST:
            return False

    return True


def is_safe_dampened(nums: list) -> bool:
    if is_safe(nums):
        return True

    for i in range(len(nums)):
        other = nums.copy()
        other.pop(i)

        if is_safe(other):
            return True

    return False


def main() -> None:
    with open("input.txt", "r") as f:
        lines = f.readlines()

    safes = 0
    safes2 = 0

    for line in lines:
        nums = list(map(int, line.strip().split()))

        if is_safe(nums):
            safes += 1
        if is_safe_dampened(nums):
            safes2 += 1

    print(f"Safes: {safes}")
    print(f"Safes, dampened: {safes2}")


if __name__ == "__main__":
    main()
