using JuMP
using Cbc

function solve(sequence::Vector{Float64})
    n = length(sequence)
    Elements = 1:n

    model = Model(Cbc.Optimizer)
    @variable(model, m >= 0, Int)
    @variable(model, s[Elements, Elements], Bin)
    # m is the maximum
    @constraint(model, [i in Elements], m >= sum(j * s[i,j] for j in Elements))
    # Each element can be put into exactly 1 bin
    @constraint(model, [i in Elements], sum(s[i,j] for j in Elements) == 1)
    # Each bin can only containt a total of 1.0
    @constraint(model, [j in Elements], sum(s[i,j] * sequence[i] for i in Elements) <= 1.0)
    @objective(model, Min, m)

    set_silent(model)
    optimize!(model)
    status = termination_status(model)
    if status == MOI.OPTIMAL
        return status, objective_value(model), value.(m), value.(s)
    else
        return status, nothing, nothing, nothing
    end
end

function extract_solution(status, obj, m, s, x)
    if status == MOI.OPTIMAL
        return trunc(Int, obj)
    else
        return ceil(sum(x))
    end
end

function print_solution(status, obj, m, s, x)
    n = length(x)
    if status == MOI.OPTIMAL
        # println(s)
        for i in 1:n
            print(x[i], "->", sum(j * s[i,j] for j in 1:n), "; ")
        end
        println()
        for j in 1:n
            print(j, "(", sum(s[i,j] * x[i] for i in 1:n), ") ")
        end
        println()
        println(m)
    else
        println(status)
    end
    solution = extract_solution(status, obj, s, m, x)
    println("Solution: ", solution)
end

# x = [0.6, 0.4, 0.6, 0.4]
x = [.1, .2, .3, .4, .5, .6, .7, .8, .9]
(status, obj, m, s) = solve(x)
print_solution(status, obj, m, s, x)
