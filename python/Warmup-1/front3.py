def front3(str):
    if len(str) < 3:
        return str * 3
    first_three = str[0] + str[1] + str[2]
    return first_three * 3
