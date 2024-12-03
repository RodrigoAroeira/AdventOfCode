import os
import sys
from pathlib import Path

if len(sys.argv) > 1:
    last = int(sys.argv[1])
else:
    last = 25


for i in range(1, last + 1):
    path = Path(f"../day{i:02d}").resolve()
    file = path / "main.cpp"
    if not os.path.exists(path):
        os.mkdir(path)
    if file.exists():
        continue

    with file.open("w", encoding="utf-8") as f:
        f.write(rf"// https://adventofcode.com/2024/day/{i}")
