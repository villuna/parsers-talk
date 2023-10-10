class ParseError(Exception):
    pass

def tag(string):
    def tag_inner(input):
        if input.startswith(string):
            return (input[:len(string)], input[len(string):])
        else:
            raise ParseError

    return tag_inner

def many0(parser):
    def many0_inner(input):
        try:
            res = []

            while True:
                parsed, input = parser(input)
                res.append(parsed)
        except ParseError:
            return (res, input)

    return many0_inner

def separated_list0(parser, separator):
    def separated_list0_inner(input):
        try:
            parsed, input = parser(input)
        except ParseError:
            return ([], input)

        res = [parsed]

        while True:
            try:
                _, input = separator(input)
            except ParseError:
                return (res, input)

            parsed, input = parser(input)
            res.append(parsed)

    return separated_list0_inner

def opt(parser):
    def opt_inner(input):
        try:
            return parser(input)
        except ParseError:
            return (None, input)

    return opt_inner

def terminated(parser1, parser2):
    def terminated_inner(input):
        res, input = parser1(input)
        _, input = parser2(input)
        return (res, input)

    return terminated_inner

def take_while(condition):
    def take_while_inner(input):
        for i, c in enumerate(input):
            if not condition(c):
                return (input[:i], input[i:])

        return (input, "")

    return take_while_inner

def map(parser, f):
    def map_inner(input):
        res, input = parser(input)
        return (f(res), input)

    return map_inner

def alt(parsers):
    def alt_inner(input):
        for p in parsers:
            try:
                return p(input)
            except ParseError:
                continue

        raise ParseError

    return alt_inner

def seq(parsers):
    def seq_inner(input):
        res = tuple()
        for p in parsers:
            parsed, input = p(input)
            res += (parsed,)

        return (res, input)

    return seq_inner

def ws(input):
    _, input = take_while(lambda c: c.isspace())(input)
    return (None, input)

def token(parser):
    def token_inner(input):
        _, input = ws(input)
        res, input = parser(input)
        _, input = ws(input)
        return (res, input)

    return token_inner

def separated_pair(p1, sep, p2):
    def sp_inner(input):
        r1, input = p1(input)
        _, input = sep(input)
        r2, input = p2(input)
        return ((r1, r2), input)
    return sp_inner

def delimited(d1, parser, d2):
    def delim_inner(input):
        _, input = d1(input)
        res, input = parser(input)
        _, input = d2(input)
        return (res, input)
    return delim_inner

def many0_recognise(parser):
    def m0r_inner(input):
        new_input = input
        i = 0

        while True:
            try:
                _, new_input = parser(new_input)
                print("new input: " + new_input)
                i += input[i:].find(new_input)
            except ParseError:
                return (input[:i], input[i:])

    return m0r_inner

def satisfy(condition):
    def satisfy_inner(input):
        if len(input) != 0 and condition(input[0]):
            return (input[0], input[1:])
        else:
            raise ParseError

    return satisfy_inner

plain_char = satisfy(lambda c: c != "\\" and c != "\"")
escaped = seq((tag("\\"), satisfy(lambda c: c in "nrt\\\"")))

string_char = alt((escaped, plain_char))
string = delimited(tag("\""), many0_recognise(string_char), tag("\""))
pair = separated_pair(string, token(tag(",")), string)
map_inner = separated_list0(pair, token(tag(",")))
map = delimited(token(tag("{")), map_inner, token(tag("}")))

print(string("\"hell yeah\""))
print(many0_recognise(string_char)("\\nhell yeah"))
#print(map_inner("\"this\" : \"is\" \n \"pretty\" : \"cool\""))
