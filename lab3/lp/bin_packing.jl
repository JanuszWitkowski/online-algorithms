using JuMP
using Cbc
# using HiGHS
# using GLPK
# using CPLEX

function solve(sequence::Vector{Float64})
    n = length(sequence)
    Elements = 1:n
    Sum = sum(sequence)

    model = Model(Cbc.Optimizer)
    # model = Model(HiGHS.Optimizer)
    # model = Model(GLPK.Optimizer)
    # model = Model(CPLEX.Optimizer)

    # @variable(model, m >= 0, Int)
    @variable(model, s[Elements, Elements], Bin)
    # m is the maximum
    # @constraint(model, [i in Elements], m >= sum(j * s[i,j] for j in Elements))
    # Each element can be put into exactly 1 bin
    @constraint(model, [i in Elements], sum(s[i,j] for j in Elements) == 1)
    # Each bin can only containt a total of 1.0
    @constraint(model, [j in Elements], sum(s[i,j] * sequence[i] for i in Elements) <= 1.0)
    # @objective(model, Min, m)
    @objective(model, Min, sum(j * s[i,j] for j in Elements, i in Elements))

    # @variable(model, m >= 0, Int)
    # @variable(model, 1 <= s[Elements] <= n, Int)
    # # m is the maximum
    # @constraint(model, [i in Elements], m >= s[i])
    # # Each bin can only containt a total of 1.0
    # @constraint(model, [j in Elements], sum(sequence[i] * (1 - ceil((s[i] - j)/n)) for i in Elements) <= 1.0)
    # @objective(model, Min, m)

    # set_silent(model)
    println("Starting solver...")
    optimize!(model)
    status = termination_status(model)
    if status == MOI.OPTIMAL
        return status, objective_value(model), value.(s)
    else
        return status, nothing, nothing
    end
end

function extract_solution(status, obj, s, x)
    if status == MOI.OPTIMAL
        # return trunc(Int, obj)
        n = length(x)
        return trunc(Int, maximum(s[i,j] * j for i in 1:n, j in 1:n))
    else
        return ceil(sum(x))
    end
end

function print_solution(status, obj, s, x)
    n = length(x)
    if status == MOI.OPTIMAL
        # println(s)
        for i in 1:n
            print(x[i], "->", trunc(Int, sum(s[i,j] * j for j in 1:n)), "; ")
        end
        println()
        for j in 1:n
            # sum = 0.0
            # for i in 1:n
            #     if s[i] == j
            #         sum += x[i]
            #     end
            # end
            mysum = sum(s[i,j] * x[i] for i in 1:n)
            if mysum > 0.0
                print(j, "(", mysum, ") ")
            end
        end
        println()
        # for j in 1:n
        #     print(sum(x[i] * (1 - ceil((abs(s[i] - j))/n)) for i in Elements))
        # end
        # println()
    else
        println(status)
    end
    solution = extract_solution(status, obj, s, x)
    println("Solution: ", solution)
end

# x = [0.6, 0.4, 0.6, 0.4]
# x = [.1, .2, .3, .4, .5, .6, .7, .8, .9]
x = [0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.28987356748636794, 0.007863033890089688, 0.007863033890089688, 0.9278562343950827, 0.9278562343950827, 0.5209463735969945, 0.5209463735969945, 0.5209463735969945, 0.5209463735969945, 0.958427652786062, 0.9278103409460138, 0.19550393283219292, 0.19550393283219292, 0.823461890411737, 0.5702096270011527, 0.5702096270011527, 0.5702096270011527, 0.42694977874454365, 0.42694977874454365, 0.42694977874454365, 0.42694977874454365, 0.42694977874454365, 0.7745079742105555, 0.7745079742105555, 0.7745079742105555, 0.7745079742105555, 0.7745079742105555, 0.7899750280485552, 0.6739114021475868, 0.6739114021475868, 0.6739114021475868, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.8547699381199185, 0.12196177827875543, 0.12196177827875543, 0.7547763750281221, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.907051753611569, 0.030048310926173105, 0.030048310926173105, 0.030048310926173105, 0.030048310926173105, 0.030048310926173105, 0.030048310926173105, 0.030048310926173105, 0.030048310926173105, 0.9195065175768564, 0.9195065175768564, 0.3555284472440362, 0.3555284472440362, 0.3555284472440362, 0.3555284472440362, 0.19467558807086416, 0.19467558807086416, 0.19467558807086416, 0.5163512985571745, 0.5163512985571745, 0.5163512985571745, 0.8829076183088354, 0.8829076183088354, 0.8829076183088354, 0.8829076183088354, 0.8829076183088354, 0.8286809044931032, 0.021734940776352518, 0.021734940776352518, 0.021734940776352518, 0.4217933580227953, 0.7058276653687452, 0.7058276653687452, 0.7058276653687452, 0.7058276653687452, 0.7058276653687452, 0.7058276653687452, 0.7058276653687452]
# x = [0.2 for _ in 1:100]
(status, obj, s) = solve(x)
print_solution(status, obj, s, x)
