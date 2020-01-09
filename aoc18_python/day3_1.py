input = open("day3_input")
sum = 0
matrix = [[0 for i in range(1000)] for x in range(1000)]
for line in input :
    line = line.split("@")
    coord_x = int(line[1].split(":")[0].split(",")[0])
    coord_y = int(line[1].split(":")[0].split(",")[1])
    width = int(line[1].split(":")[1].split("x")[0])
    height = int(line[1].split(":")[1].split("x")[1])
    for h in range(height) :
        for w in range(width) :
            if matrix[coord_y + h][coord_x + w] == 1 :
                sum += 1
            matrix[coord_y + h][coord_x + w] += 1
input.close()
input = open("day3_input")
for line in input :
    line = line.split("@")
    coord_x = int(line[1].split(":")[0].split(",")[0])
    coord_y = int(line[1].split(":")[0].split(",")[1])
    width = int(line[1].split(":")[1].split("x")[0])
    height = int(line[1].split(":")[1].split("x")[1])
    id = line[0]
    free = True
    for h in range(height) :
        for w in range(width) :
            if matrix[coord_y + h][coord_x + w] != 1 :
                free = False
    if free :
        print id
