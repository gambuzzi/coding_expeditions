# @title GPU and jit fixed matrix { form-width: "200px" }

import timeit

import numpy as np
from numba import cuda, njit

ench = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"

img = """#..#.
#....
##..#
..#..
..###"""


@njit
def parse_input(img):
    ret = np.full((200, 200), False)
    for y, row in enumerate(img.splitlines()):
        for x, c in enumerate(row):
            ret[50 + x, 50 + y] = c == "#"
    return ret


def pure_step(grid, default=0):
    new_grid = np.full((200, 200), False)
    for x in range(grid.shape[0]):
        for y in range(grid.shape[1]):
            n = 0
            for dx, dy in (
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (0, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ):
                n *= 2
                if (not 0 <= x + dx < grid.shape[0]) or (
                    not 0 <= y + dy < grid.shape[1]
                ):
                    n += default
                else:
                    n += 1 if grid[x + dx, y + dy] else 0
            new_grid[x, y] = ench[n] == "#"
    return new_grid


def pure_part2(img, ench):
    grid = parse_input(img)

    for i in range(50):
        grid = pure_step(grid, default=i % 2 if ench[0] == "#" else 0)

    return np.count_nonzero(grid)


print("pure part2", pure_part2(img, ench))
print(
    timeit.timeit(
        "pure_part2(img, ench)", "from __main__ import pure_part2, img, ench", number=1
    )
)


@njit
def step(grid, default=0):
    new_grid = np.full((200, 200), False)
    for x in range(grid.shape[0]):
        for y in range(grid.shape[1]):
            n = 0
            for dx, dy in (
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (0, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ):
                n *= 2
                if (not 0 <= x + dx < grid.shape[0]) or (
                    not 0 <= y + dy < grid.shape[1]
                ):
                    n += default
                else:
                    n += 1 if grid[x + dx, y + dy] else 0
            new_grid[x, y] = ench[n] == "#"
    return new_grid


grid = parse_input(img)
# pprint(grid)
grid = step(grid)
# pprint(grid)
grid = step(grid, default=1 if ench[0] == "#" else 0)
# pprint(grid)
print("jit part1", np.count_nonzero(grid))

# part 2


@njit
def part2(img, ench):
    grid = parse_input(img)

    for i in range(50):
        grid = step(grid, default=i % 2 if ench[0] == "#" else 0)

    return np.count_nonzero(grid)


print("jit part2", part2(img, ench))
print(
    timeit.timeit(
        "part2(img, ench)", "from __main__ import part2, img, ench", number=10
    )
    / 10
)


@cuda.jit
def GPUstep(ingird, outgrid, ench, default):
    x = cuda.grid(1)
    y = x // 200
    x %= 200
    # size = cuda.gridsize(2)
    if x < ingird.shape[0] and y < ingird.shape[1]:
        n = 0
        for dx, dy in (
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ):
            n *= 2
            if (not 0 <= x + dx < ingird.shape[0]) or (
                not 0 <= y + dy < ingird.shape[1]
            ):
                n += default
            else:
                n += 1 if ingird[x + dx, y + dy] else 0
        outgrid[x][y] = ench[n]


def part2GPU(img, ench):
    BLK = 2500  # 200*200/THS
    THS = 16

    grid = cuda.to_device(parse_input(img))
    new_grid = cuda.to_device(np.full((200, 200), False))
    d_ench = cuda.to_device(np.fromiter((x == "#" for x in ench), bool))

    for i in range(50):
        GPUstep[BLK, THS](grid, new_grid, d_ench, i % 2 if ench[0] == "#" else 0)
        grid, new_grid = new_grid, grid

    return np.count_nonzero(grid)


print("GPU part2", part2GPU(img, ench))
print(
    timeit.timeit(
        "part2GPU(img, ench)", "from __main__ import part2GPU, img, ench", number=1000
    )
    / 1000
)
