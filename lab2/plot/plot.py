#!/usr/bin/python3
from os import listdir
from sys import argv
from os.path import isfile, join
from typing import List
from numbers import Number
import matplotlib.pyplot as plt
import numpy as np
from data import proper_name, name_without, open_data_from_csv

def get_files_from_dir (path: str):
    return [join(path, f) for f in listdir(path) if isfile(join(path, f))]

def create_plot (files: List[str], search: str, prefix: str, condition: str, n: str):
    files_searched = [f for f in files if condition in f and n in f and search in f]
    plt.figure(figsize=(16, 8))
    for filename in files_searched:
        ks, data = open_data_from_csv(filename)
        plt.plot(np.array(ks).astype('str'), data, label=name_without(proper_name(filename), [search, condition, n, "data"]))
    plt.xlabel("Wielkość cache [k]")
    plt.ylabel("Średni koszt operacji access(x)")
    plt.legend()
    plt.title(search + "_" + n + " (" + condition + ")")
    plt.savefig(prefix + search + "_" + n + condition + '.png')


if __name__ == "__main__":
    subdir = "plots/"
    iterations = "_r" + "30000" + "_"
    if len(argv) > 1:
        path = argv[1]
        files = get_files_from_dir(path)
        ns = ["n20", "n30", "n40", "n50", "n60", "n70", "n80", "n90", "n100"]
        searches = ["_First-In-First-Out_", "_Flush-When-Full_", "_Least-Frequently-Used_", "_Least-Recently-Used_", "_Random_", "_Random-Markup-Algorithm_",
                    "_uniform", "_geometric", "_harmonic", "_diharmonic"]
        for search in searches:
            for n in ns:
                print(search + " " + n)
                create_plot(files, search, subdir, iterations, n)
