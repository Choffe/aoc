# tactic: save all destinations in a list, 
# create a matrix with coordinates, loop over and calculate closest destination and write it as value
# go through matrix and count number of occurencies of each letter, maybe do this in prev loop
# go through the edge. any letter there cannot be a winner. delet them



input = open("day6_input")
destinations = []
for line in input:
    x = line.strip().split(",")[0]
    x = int(x)
    y = line.strip().split(",")[1]
    y = int(y)
    destinations.append((x,y))
print destinations


matrix_size = 1000
limit = 10000
matrix = [[-1 for i in range(matrix_size)] for j in range(matrix_size)]
infinite = []
area = 0
for y in range(matrix_size) :
    for x in range(matrix_size) :
        # calc manhattan dist to every dist
        min_dist = matrix_size * 2
        min_dist_id = -1
        sum_dist = 0
        for i in range(len(destinations)) :
            (dest_x, dest_y) = destinations[i]
            current_dist = abs(dest_x - x) + abs(dest_y - y)
            if current_dist < min_dist :
                min_dist = current_dist
                min_dist_id = i
            elif current_dist == min_dist and i != min_dist_id :
                min_dist_id = -1
            sum_dist += current_dist
        if (sum_dist < limit) :
            area += 1
        matrix[y][x] = min_dist_id
        border = (x == matrix_size-1 or x == 0) or (y == matrix_size - 1 or y == 0)  
        if min_dist_id != -1 and border and min_dist_id not in infinite :
            infinite.append(min_dist_id)

for y in range(matrix_size) :
    for x in range(matrix_size) :
        if matrix[y][x] in infinite :
            matrix[y][x] = -2

count = {}
for y in range(matrix_size) :
    for x in range(matrix_size) :
        if matrix[y][x] >= 0:
            if matrix[y][x] in count.keys() :
                count[matrix[y][x]] += 1
            else :
                count[matrix[y][x]] = 1
print count 
print max(count.values()) 
print area        
        
