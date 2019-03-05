"""
Problem:
Given a string and a non-negative int n, we'll say that the front of the string is the first 3 chars, or whatever is there if the string is less than length 3. Return n copies of the front;
"""


def front_times(str, n):
    if len(str) >= 3:
        firstChar = str[0]
        secondChar = str[1]
        thirdChar = str[2]
        firstThree_lst = [firstChar, secondChar, thirdChar]
        firstThree = "".join(firstThree_lst)
        words = []
        for i in range(n):
            words.append(firstThree)
        final_string = "".join(words)
        return final_string
    elif len(str) == 2:
        firstChar = str[0]
        secondChar = str[1]
        firstTwo_lst = [firstChar, secondChar]
        firstTwo = "".join(firstTwo_lst)
        words = []
        for i in range(n):
            words.append(firstTwo)
        final_string = "".join(words)
        return final_string
    elif len(str) == 1:
        words = []
        for i in range(n):
            words.append(str[0])
        final_string = "".join(words)
        return final_string
    else:
        return str
