def string_match(a, b):
    substrs = 0
    try:
        for i in range(0, len(a) - 1):
            if a[i] + a[i + 1] == b[i] + b[i + 1]:
                substrs += 1
    except IndexError:
        pass
    return substrs
