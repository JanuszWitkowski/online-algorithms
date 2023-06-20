#!/usr/bin/python3
import matplotlib.pyplot as plt
import csv
import numpy as np

rows = []

D = [16, 32, 64, 128, 256]
P = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5]
cost = np.zeros((len(D),len(P)))
copies = np.zeros((len(D),len(P)))
costP = np.zeros((len(P),len(D)))
copiesP = np.zeros((len(P),len(D)))

if __name__ == "__main__":
    with open("../results/4/total.csv") as csvfile:
            reader = csv.reader(csvfile, delimiter=';')
            for row_ in reader:
                rows.append(row_)  # d, p, cost, copies
    
    # print("len: " + str(len(rows)))
    # print(rows)

    # plt.figure(figsize=(16, 8))
    # plt.xlabel("P")
    # plt.ylabel("Cost")
    costs_l = []
    copies_l = []
    for di in range(5):
        costs_l.append([])
        copies_l.append([])
    # for di in range(5):
    #     for pi in range(6):
    #         costs_l[di].append(0.0)
    #         copies_l[di].append(0.0)
    # for di in range(5):
        for pi in range(6):
            # print(str(di+pi))

            costs_l[di].append(float(rows[6*di+pi][2]))
            copies_l[di].append(float(rows[6*di+pi][3]))
            # costs_l[di][pi] = (float(rows[di+pi][2]))
            # copies_l[di][pi] = (float(rows[di+pi][3]))

            # print("rows[di+pi][2]:", rows[di+pi][2])
            # print("rows[di+pi][3]:", rows[di+pi][3])
        # plt.plot(costs, label="D="+rows[di*6][0])
    # plt.legend()
    # plt.savefig("test.png")
    with open("costs.csv", 'w') as f:
        for row_ in costs_l:
            to_write = ""
            for cost_ in row_:
                # f.write(str(cost_) + ";")
                to_write = to_write + str(cost_) + ";"
            f.write(to_write[:-1] + "\n")
    with open("copies.csv", 'w') as f:
        for row_ in copies_l:
            to_write = ""
            for copy in row_:
                # f.write(str(copy) + ";")
                to_write = to_write + str(copy) + ";"
            f.write(to_write[:-1] + "\n")
    

    with open('costs.csv') as csvfile:
            reader = csv.reader(csvfile, delimiter=';')
            for i,row in enumerate(reader):
                # print("row: "+str(row))
                for j in range(len(P)):
                    # print("cost[i][j]: ", cost[i][j])
                    cost[i][j] = float(row[j])
                    costP[j][i] = float(row[j])

    with open('copies.csv') as csvfile:
            reader = csv.reader(csvfile, delimiter=';')
            for i,row in enumerate(reader):
                for j in range(len(P)):
                    copies[i][j] = float(row[j])
                    copiesP[j][i] = float(row[j])

    plt.figure(figsize=(16,8))
    for i,d in enumerate(D):
        plt.plot(P, cost[i][:], label="D = "+str(d))
    plt.xlabel('Prawdopodobieństwo żądania WRITE (p)')
    plt.ylabel('Średni koszt żądania w algorytmie')
    plt.legend()
    plt.savefig("koszt_D.png", dpi=300)
    plt.close()

    plt.figure(figsize=(16,8))
    for i,d in enumerate(D):
        plt.plot(P, copies[i][:], label="D = "+str(d))
    plt.xlabel('Prawdopodobieństwo żądania WRITE (p)')
    plt.ylabel('Średnie maksimum liczby kopii')
    plt.legend()
    plt.savefig("kopie_D.png", dpi=300)
    plt.close()

    plt.figure(figsize=(16,8))
    for j,p in enumerate(P):
        plt.plot(D, costP[j][:], label="P = "+str(p))
    plt.xlabel('Koszt skopiowania zasobu (D)')
    plt.ylabel('Średni koszt żądania w algorytmie')
    plt.legend()
    plt.savefig("koszt_P.png", dpi=300)
    plt.close()

    plt.figure(figsize=(16,8))
    for j,p in enumerate(P):
        plt.plot(D, copiesP[j][:], label="P = "+str(p))
    plt.xlabel('Koszt skopiowania zasobu (D)')
    plt.ylabel('Średnie maksimum liczby kopii')
    plt.legend()
    plt.savefig("kopie_P.png", dpi=300)
    plt.close()
    
    # ds = set()
    # ps = set()
    # for row in rows:
    #     ds.add(row[0])
    #     ps.add(row[1])
    
    # plt.figure(figsize=(16, 8))
    # plt.xlabel("P")
    # plt.ylabel("Cost")
    # for d in ds:
    #     costs = []
    #     for row in rows:
    #         if row[0] == d:
    #             costs.append(row[2])
    #     plt.plot(list(ps), costs, label="D="+str(d))
    # plt.savefig("test.png")


# def get_files_from_dir (path: str):
#     return [join(path, f) for f in listdir(path) if isfile(join(path, f))]

# def create_plot (files: List[str], search: str, prefix: str, condition: str, n: str):
#     files_searched = [f for f in files if condition in f and n in f and search in f]
#     plt.figure(figsize=(16, 8))
#     for filename in files_searched:
#         ks, data = open_data_from_csv(filename)
#         plt.plot(np.array(ks).astype('str'), data, marker='o', label=name_without(proper_name(filename), [search, condition, n, "data"]))
#     plt.xlabel("Wielkość cache [k]")
#     plt.ylabel("Średni koszt operacji access(x)")
#     plt.legend()
#     plt.title(search + "_" + n + " (" + condition + ")")
#     plt.savefig(prefix + search + "_" + n + condition + '.png')


# if __name__ == "__main__":
#     subdir = "plots/"
#     iterations = "_r" + "30000" + "_"
#     if len(argv) > 1:
#         path = argv[1]
#         files = get_files_from_dir(path)
#         # ns = ["n20", "n30", "n40", "n50", "n60", "n70", "n80", "n90", "n100"]
#         # searches = ["_First-In-First-Out_", "_Flush-When-Full_", "_Least-Frequently-Used_", "_Least-Recently-Used_", "_Random_", "_Random-Markup-Algorithm_",
#         #             "_uniform", "_geometric", "_harmonic", "_diharmonic"]
#         # for search in searches:
#         #     for n in ns:
#         #         print(search + " " + n)
#         #         create_plot(files, search, subdir, iterations, n)
