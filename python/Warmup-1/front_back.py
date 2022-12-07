def front_back(str):
    if len(str) < 2:
        return str
    chars = list(str)
    first = chars[0]
    last = chars[-1]
    chars[0] = last
    chars[-1] = first
    return "".join(chars)
