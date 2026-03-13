import matplotlib.pyplot as plt
import numpy as np

INPUT_FILE = "results/triangles.txt"


def parse_triangles(file_path) -> [(int, int)]:
    with open(file_path, "r") as file:
        triangle_strings = file.readlines()
        triangle_count = len(triangle_strings)

        print(f"Loaded {triangle_count} triangles")
        triangles = np.zeros(triangle_count * 2, dtype=int).reshape(triangle_count, 2)

        # load into numpy array
        for i in range(triangle_count):
            triangle_string = triangle_strings[i]
            a, b = triangle_string.removesuffix("\n").split(",")
            triangles[i, 0] = a
            triangles[i, 1] = b

        sorted_rows = np.sort(triangles, axis=1)

        print(f"Parsed {triangle_count} triangles and sorted into numpy array")

        # cull duplicates
        unique_triangles = np.unique(sorted_rows, axis=0)

        print(f"Culled {triangle_count - len(unique_triangles)} duplicate entries")

        return unique_triangles


def main() -> None:
    fig = plt.figure(figsize=(12, 8))
    ax = fig.add_subplot(1, 1, 1)

    triangles = parse_triangles(INPUT_FILE)

    for triangle in triangles:
        a, b = triangle
        ax.scatter(a, b, color="tab:blue")

    plt.show()


if __name__ == "__main__":
    main()
