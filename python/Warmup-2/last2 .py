def last2(str):
    appearances = 0
    for i in range(len(str) - 2):
        if i != len(str) - 1:
            if str[i] + str[i + 1] == str[-2:]:
                appearances += 1
    return appearances
