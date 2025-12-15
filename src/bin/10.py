import pulp
import numpy as np
from pathlib import Path


def solve_integer_min_sum(A, b):
    """
    Finds an integer vector x such that Ax = b, x >= 0, and sum(x) is minimized.

    Args:
        A (list or np.array): The coefficient matrix (m rows x n columns).
        b (list or np.array): The right-hand side vector (size m).

    Returns:
        list: The integer solution vector x if optimal.
        None: If no solution exists.
    """
    # Ensure inputs are numpy arrays for easier shape handling
    A = np.array(A)
    b = np.array(b)

    m, n = A.shape

    if len(b) != m:
        raise ValueError(f"Shape mismatch: A has {m} rows but b has length {len(b)}")

    # 1. Initialize the Model
    # We use LpMinimize because we want to minimize the sum of x
    prob = pulp.LpProblem("Minimize_Vector_Sum", pulp.LpMinimize)

    # 2. Define Variables
    # Create n integer variables x_0, x_1, ..., x_{n-1}
    # lowBound=0 enforces x >= 0
    # cat='Integer' enforces x is an integer
    x_vars = [pulp.LpVariable(f"x_{i}", lowBound=0, cat="Integer") for i in range(n)]

    # 3. Define Objective Function
    # Minimize sum(x)
    prob += pulp.lpSum(x_vars)

    # 4. Define Constraints
    # Ax = b implies that for each row i, dot(A[i], x) == b[i]
    for i in range(m):
        prob += pulp.lpSum([A[i][j] * x_vars[j] for j in range(n)]) == b[i]

    # 5. Solve the problem
    # The default solver (usually CBC) is used. You can pass explicit solvers (e.g., pulp.GLPK()) if needed.
    status = prob.solve(
        pulp.PULP_CBC_CMD(msg=False)
    )  # msg=False suppresses solver logs

    # 6. Process Results
    if pulp.LpStatus[status] == "Optimal":
        # Return the values as a list of integers
        return [int(var.varValue) for var in x_vars]
    else:
        # Problem is Infeasible or Unbounded
        print(f"Problem status: {pulp.LpStatus[status]}")
        return None


def parse_line(line: str):
    linesplit = line.split(" ")
    lights = linesplit[0]
    buttons = linesplit[1:-1]
    buttons = [[int(x) for x in y[1:-1].split(",")] for y in buttons]
    joltages = [int(x) for x in linesplit[-1].strip()[1:-1].split(",")]
    return lights, buttons, joltages


def parse_input(fname: Path):
    actions = []
    joltages = []
    with open(fname, "r") as f:
        for line in f.readlines():
            _, action, jolts = parse_line(line)
            actions.append(action)
            joltages.append(jolts)
    return actions, joltages


def actions_to_matrix(actions: list[int], n: int):
    A = np.zeros((n, len(actions)))
    for i, a in enumerate(actions):
        A[a, i] = 1
    return A


if __name__ == "__main__":
    example_file = Path(__file__).parent.parent.parent / "data/examples/10.txt"
    input_file = Path(__file__).parent.parent.parent / "data/inputs/10.txt"
    actions, joltages = parse_input(example_file)
    checksum = 0
    for action, jolts in zip(actions, joltages):
        A = actions_to_matrix(action, len(jolts))
        x = solve_integer_min_sum(A, jolts)
        checksum += np.sum(x)
    assert checksum == 463, f"Wrong answer (had {checksum})!"
    # actually solve
    actions, joltages = parse_input(input_file)
    checksum = 0
    for action, jolts in zip(actions, joltages):
        A = actions_to_matrix(action, len(jolts))
        x = solve_integer_min_sum(A, jolts)
        checksum += np.sum(x)
    print(f"Sum of bests = {checksum}")
