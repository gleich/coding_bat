def string_splosion(str):
    str_chunk = ""
    new_str = ""
    for char in str:
        str_chunk += char
        new_str += str_chunk
    return new_str
