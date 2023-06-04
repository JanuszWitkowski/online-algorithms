from os import listdir
from os.path import isfile, join

DIRNAME = "../results/"
EXPV_AVG = "expv_avg_"


def get_files_from_dir (path: str, restriction: str):
    return [join(path, f) for f in listdir(path) if isfile(join(path, f)) and restriction in f]

def combine_files(dirname: str, files_regex: str, filename: str):
    print("Combining files...")
    files = get_files_from_dir(dirname, files_regex)
    with open(filename, 'w') as result:
        for fname in files:
            with open(fname) as f:
                data = [float(line.rstrip()) for line in f]
            result.write(fname + ":\t" + str(data[0]) + "\n")


if __name__ == "__main__":
    print("combine.py")
    combine_files(DIRNAME, EXPV_AVG, EXPV_AVG + ".txt")

