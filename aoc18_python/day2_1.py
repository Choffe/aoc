inputfile = open("day2_input")
input = []
for x in inputfile :
    input.append(x)
for j in range(len(input)) :
    line = input[j]
    for k in range(len(input)) :
        if j == k :
            continue
        other_line = input[k]
        num_diff = 0
        # print line, other_line
        for i in range(len(line)) :
            if line[i] != other_line[i] :
                # print line[i], other_line[i], i
                num_diff += 1
            if num_diff > 1 :
                break
        if num_diff == 1 :
            print line, other_line
            print num_diff
pebjqsalrdnckzfihvtxysomg
