input = open("day8_input")

class Node :
    childs = 0
    nbr_meta = 0
    meta = []


def parse_child(list, index) :
    childs = int(list[index])
    nbr_meta = int(list[index+1])
    print "nbr childs ", childs, "nbr meta" ,nbr_meta
    if childs == 0 :
        sum = 0
        for n in range(nbr_meta) :
            sum += int(list[index + 2 + n])
            print "adding " ,int(list[index + 2 + n])
        print "returning " , (sum, index + 2 + nbr_meta)
        return sum, index + 2 + nbr_meta
    else :
        new_index = index + 2
        sum = 0
        for c in range(childs) :
            (s , p) = parse_child(list, new_index)
            print " child processed, adding ", s, "and new index is " , p
            sum += s
            new_index = p
        for n in range(nbr_meta) :
            print " processing metadata, adding ", int(list[new_index]), " at index" , new_index
            sum += int(list[new_index])
            new_index = new_index + 1
        return (sum, new_index)

for line in input :
    print parse_child(line.split(), 0)