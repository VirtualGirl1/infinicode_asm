
struct token {
    tkn_type: token_type,
    value: String,
    size: i8,
}

enum token_type {
    label,
    opcode,
    reg,
    abs,
    offset,
    preproc,
    data,
    global
}