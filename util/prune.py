with open('words.txt', encoding="utf-8") as f:
    word_list = set([line.rstrip('\n') for line in f])

glove_unpruned = open("glove.840B.300d.txt", encoding="utf-8")
output = open("glove.pruned.300d.txt", "w", encoding="utf-8")
print(word_list)

for line in glove_unpruned:
    if (line.split()[0] in word_list):
        output.writelines(line)

glove_unpruned.close()
output.close()
