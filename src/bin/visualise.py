import matplotlib.pyplot as plt
from pathlib import Path

with open(Path(__file__).parent.parent.parent / "data/inputs/09.txt") as f:
    lines = f.readlines()
    coords = [l.split(",") for l in lines]
    coords = [(int(c[0]), int(c[1])) for c in coords]

plt.plot(*zip(*coords))
plt.show()
