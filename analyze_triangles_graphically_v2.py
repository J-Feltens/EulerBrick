import matplotlib.pyplot as plt
import numpy as np

INPUT_FILE = "results/triangles.txt"
COMPRESSION_FACTOR = 100


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

    max_side_length = np.max(triangles)
    print(f"Max side length: {max_side_length}")

    im = np.zeros((max_side_length, max_side_length))
    for (a, b) in triangles:
        im[a - 1, b - 1] = 1

    im_reduced = np.zeros((max_side_length // COMPRESSION_FACTOR, max_side_length // COMPRESSION_FACTOR))
    for y in range(max_side_length // COMPRESSION_FACTOR):
        for x in range(max_side_length // COMPRESSION_FACTOR):
            im_reduced[y, x] = np.sum(im[
                                          y * COMPRESSION_FACTOR:(y + 1) * COMPRESSION_FACTOR,
                                          x * COMPRESSION_FACTOR: (x + 1) * COMPRESSION_FACTOR])

    print(f"Reduced image size: {im_reduced.shape}")
    print(f"Maximum pixel value: {np.max(im_reduced)}")

    ax.imshow(im_reduced, cmap="gray")
    plt.savefig('euler_analysis.png')
    plt.show()


if __name__ == "__main__":
    main()
