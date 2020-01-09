input = open("day5_input")

org = []

for line in input :
    for char in line :
        org.append(char)

char_list = []
for num in range(26) :
    char_list.append(chr(num + 65))
res = {}
for c in char_list :
    word = org[:]
    word = filter(lambda a: a != c and a != chr(ord(c) + 32), word)
 
    reaction = False
    i = 0
    while (i < len(word) - 1) :
        if (ord(word[i]) - ord(word[i + 1]) == 32 or ord(word[i]) - ord(word[i + 1]) == -32) :  #lower and upper case differ by 32
            reaction = True
            del word[i]
            del word[i]
            i = 0
        else :
            i += 1

    res[c] = len(word)

print res
print min(res.values())
