"""
Problem:
Given a string and a non-negative int n, return a larger string that is n copies of the original string.
"""


def string_times(str, n):
    words = []
    for i in range(n):
        words.append(str)
    final_string = "".join(words)
    return final_string
