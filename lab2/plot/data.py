from typing import List

def proper_name (filename: str) -> str:
    # name = filename
    name = filename[:-4]
    l = len(name)
    idx = 0
    while idx < l and name[-idx] != '/':
        idx += 1
    if idx == l:
        return name
    return name[-idx+1:]

def name_without(s: str, to_remove: List[str]) -> str:
    for forbidden in to_remove:
        s = s.replace(forbidden, '')
    # print(s)
    if s[0] == "_":
        s = s[1:]
    return s

def open_integers(filename):
    with open(filename) as file:
        numbers = [int(line.rstrip()) for line in file]
    return numbers

def open_floats(filename):
    with open(filename) as file:
        numbers = [float(line.rstrip()) for line in file]
    return numbers

# def open_integers_csv(filename):
#     with open(filename) as file:
#         numbers = [int(line.rstrip()) for line in file]
#     return numbers

def open_data_from_csv(filename):
    with open(filename) as file:
        lines = [line.rstrip() for line in file]
    splits = [line.split(';') for line in lines]
    numbers = [int(line[0]) for line in splits]
    data = [float(line[1]) for line in splits]
    return numbers, data
