def missing_char(str, n):
    chars = list(str)
    del chars[n]
    return "".join(chars)
