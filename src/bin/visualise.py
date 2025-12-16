import matplotlib.pyplot as plt
from pathlib import Path

with open(Path(__file__).parent.parent.parent / "data/inputs/09.txt") as f:
    lines = f.readlines()
    coords = [l.split(",") for l in lines]
    coords = [(int(c[0]), int(c[1])) for c in coords]

plt.plot(*zip(*coords))
plt.axvline(52177, c="r")
plt.axvline(98198, c="r")
plt.axhline(50248, c="r")
plt.axhline(98329, c="r")
plt.axvline(5570, c="g")
plt.axvline(94693, c="g")
plt.axhline(68792, c="g")
plt.axhline(50233, c="g")
plt.show()
