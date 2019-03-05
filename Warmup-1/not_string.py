"""
Problem:
Given a string, return a new string where "not " has been added to the front. However, if the string already begins with "not", return the string unchanged.
"""


def not_string(str):
    words_lst = str.split(" ")
    if words_lst[0] == "not":
        return str
    else:
        new_str = "not " + str
        return new_str
