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

def open_integers(filename):
    with open(filename) as file:
        numbers = [int(line.rstrip()) for line in file]
    return numbers

def open_floats(filename):
    with open(filename) as file:
        numbers = [float(line.rstrip()) for line in file]
    return numbers
