def not_string(str):
    if str.startswith("not"):
        return str
    return "not " + str
