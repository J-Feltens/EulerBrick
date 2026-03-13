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
    triangles = parse_triangles(INPUT_FILE)

    for (a1, b1) in triangles:
        for (a2, b2) in triangles:
            for (a3, b3) in triangles:
                # a_1 = a_2 ~~\land~~ b_1 = b_3 ~~\land~~ b_2 = a_3
                if a1 == a2 and b1 == b3 and b2 == a3:
                    print(f"Found euler brick! ({a1}, {b1}, {b2})")


if __name__ == "__main__":
    main()
