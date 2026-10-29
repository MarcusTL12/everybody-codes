
function part1()
    open("$(homedir())/ec-input/everybody_codes_e2024_q2_p1.txt") do io
        lines = Iterators.Stateful(eachline(io))

        runes = popfirst!(lines)
        popfirst!(lines)
        inscription = popfirst!(lines)

        runelist = split(runes, ":")[2]
        runelist = replace(runelist, "," => "|")
        reg = Regex(runelist)

        count(true for _ in eachmatch(reg, inscription, overlap=true))
    end
end

function count_symbols(reg, s)
    bitmap = falses(length(s))

    for m in eachmatch(reg, s, overlap=true)
        start = m.match.offset + 1
        stop = start + m.match.ncodeunits - 1
        bitmap[start:stop] .= true
    end

    count(bitmap)
end

function part2()
    open("$(homedir())/ec-input/everybody_codes_e2024_q2_p2.txt") do io
        lines = Iterators.Stateful(eachline(io))

        runes = popfirst!(lines)
        popfirst!(lines)

        runelist = split(runes, ":")[2]
        runelist = replace(runelist, "," => "|")
        reg = Regex(runelist * "|" * reverse(runelist))

        sum(count_symbols(reg, l) for l in lines)
    end
end
