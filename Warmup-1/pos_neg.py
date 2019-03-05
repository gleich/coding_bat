"""
Problem:
Given 2 int values, return True if one is negative and one is positive. Except if the parameter "negative" is True, then return True only if both are negative.
"""


def pos_neg(a, b, negative):
    if negative == False and b > 0 and a < 0:
        return True
    elif negative == False and b < 0 and a > 0:
        return True
    elif negative == True and b < 0 and a < 0:
        return True
    else:
        return False
