"""
Problem:
Given a string, return a new string where the first and last chars have been exchanged.
"""


def front_back(str):
    if len(str) == 2:
        firstChar = str[0]
        lastChar = str[-1]
        return lastChar + firstChar
    elif len(str) < 2:
        return str
    else:
        firstChar = str[0]
        lastChar = str[-1]
        return lastChar + str[1:-1] + firstChar
