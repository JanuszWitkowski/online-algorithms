#!/usr/bin/python3
from os import listdir
from sys import argv
from os.path import isfile, join
from typing import List
from numbers import Number
import matplotlib.pyplot as plt
import numpy as np
from data import proper_name, open_integers, open_floats

def get_files_from_dir (path: str):
    return [join(path, f) for f in listdir(path) if isfile(join(path, f))]

def create_plot (files: List[str], search: str, ns: List[Number]):
    files_searched = [f for f in files if search in f]
    # plt.figure().set_figwidth(18)
    plt.figure(figsize=(16, 8))
    for filename in files_searched:
        # plt.plot(ns, open_floats(filename), label=proper_name(filename))
        plt.plot(np.array(ns).astype('str'), open_floats(filename), label=proper_name(filename))
    plt.legend()
    plt.title(search)
    plt.savefig(search + '.png')
    # plt.show()


if __name__ == "__main__":
    if len(argv) > 1:
        path = argv[1]
        files = get_files_from_dir(path)
        ns = open_integers(join(path, "ns.txt"))
        searches = ["_classic", "_move-to-front", "_transpose", "_count",
                    "_uniform", "_geometric", "_harmonic", "_twoharmonic"]
        # searches = ["_classic", "_uniform"]
        for search in searches:
            # print(search)
            create_plot(files, search, ns)
