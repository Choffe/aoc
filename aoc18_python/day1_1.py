input = open("day1_input.txt")
sum = 0
list = []
for line in input :
    list.append(float(line.strip()))
dictionary = {}
index = 0
while (1) :
    if index >= len(list) :
        index = 0

    sum += list[index]
    if sum in dictionary :
        break
    dictionary[sum] = 1
    index += 1

print sum
