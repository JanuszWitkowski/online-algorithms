from data import name_without
from plot import proper_name, get_files_from_dir

doktur_prefix = "DOKTURpaging_res_"
doktur_format = ".txt"
my_prefix = "data_r30000_"
my_format = ".csv"
alg = "_Random-Markup-Algorithm_"
name_uniform = "uniform"
name_geometric = "geometric"
name_harmonic = "harmonic"
name_diharmonic = "diharmonic"
path = "../results/noRMA/res2/"

if __name__ == "__main__":
    files = get_files_from_dir(path)
    for filename in files:
        with open(filename) as f:
            lines = [line.rstrip() for line in f]
        useful_lines = lines[1:]
        splits = [line.split(';') for line in useful_lines]
        ks = [int(line[0]) for line in splits]
        uniform = [float(line[1]) for line in splits]
        harmonic = [float(line[2]) for line in splits]
        diharmonic = [float(line[3]) for line in splits]
        geometric = [float(line[4]) for line in splits]
        n = name_without(proper_name(filename), [doktur_prefix, doktur_format])

        file_uniform = my_prefix + n + alg + name_uniform + my_format
        file_geometric = my_prefix + n + alg + name_geometric + my_format
        file_harmonic = my_prefix + n + alg + name_harmonic + my_format
        file_diharmonic = my_prefix + n + alg + name_diharmonic + my_format

        with open(file_uniform, 'w') as f:
            for i in range(len(ks)):
                to_write = str(ks[i]) + ";" + str(uniform[i]) + "\n"
                f.write(to_write)
        with open(file_geometric, 'w') as f:
            for i in range(len(ks)):
                to_write = str(ks[i]) + ";" + str(geometric[i]) + "\n"
                f.write(to_write)
        with open(file_harmonic, 'w') as f:
            for i in range(len(ks)):
                to_write = str(ks[i]) + ";" + str(harmonic[i]) + "\n"
                f.write(to_write)
        with open(file_diharmonic, 'w') as f:
            for i in range(len(ks)):
                to_write = str(ks[i]) + ";" + str(diharmonic[i]) + "\n"
                f.write(to_write)
        # 
