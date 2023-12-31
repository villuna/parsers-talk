// -- Simple parsers --

// integer ::= '0' | onenine { digit }
export function integer(input) {
    if (input[0] == '0') {
        return {
            value: 0,
            input: input.slice(1),
        }
    } else if (input[0] >= '1' && input[0] <= '9') {
        let length = 1;

        while (length < input.length && (input[length] >= '0' && input[length] <= '9')) {
            length++;
        }

        return {
            value: Number(input.slice(0, length)),
            input: input.slice(length),
        };
    } else {
        throw "integer: input does not start with a valid integer";
    }
}

// Parses the given string, returns error otherwise
//
// string("hello")("hello world") == { input: " world", value: "hello"}
// string("hello")("konnichiwa sekai") == error
export function string(str) {
    return (input) => {
        if (input.startsWith(str)) {
            return {
                value: str,
                input: input.slice(str.length),
            };
        }

        throw `string: expected '${str}'`;
    };
}

// Parses a single character.
export function char(input) {
    if (input.length == 0) {
        throw "char: input is empty";
    }

    return {
        value: input[0],
        input: input.slice(1),
    };
}

// Parses a single character if it satisfies the predicate
// 
// e.g, satisfy((c) => c >= '0' && c <= '9')
// parses a single digit, and returns error otherwise.
export function satisfy(predicate) {
    return (input) => {
        let result = char(input);

        if (!predicate(result.value)) {
            throw "satisfy: character does not satisfy the predicate";
        }

        return result;
    }
}

// -- Combinators / higher order export functions --

// sequence: takes in a list of parsers.
// parses each one in sequence and returns all of their results as a list.
// fails if any one of component parsers fails.
export function seq(...parsers) {
    return (input) => {
        let res = [];

        for (let parser of parsers) {
            let value;
            ({value, input} = parser(input));
            res.push(value);
        }

        return {
            value: res,
            input,
        };
    }
}

// either: takes in a list of parsers.
// tries each one of them until one succeeds, and then returns its result.
// fails if none of the parsers matched.
export function either(...parsers) {
    return (input) => {
        for (let parser of parsers) {
            try {
                return parser(input);
            } catch {}
        }

        throw "either: none of the parsers succeeded";
    }
}

// zeroOrMore: runs a parser over and over until it fails, collecting
// its results in a list and returning that at the end.
//
// DO NOT use this with a parser that matches the empty string!
// it will never fail and therefore infinite loop
export function zeroOrMore(parser) {
    return (input) => {
        let value;
        let res = [];

        while (true) {
            try {
                ({value, input} = parser(input));
                res.push(value);
            } catch {
                break;
            }
        }

        return {
            value: res,
            input
        };
    }
}

export function delimited(d1, parser, d2) {
    return (input) => {
        let value;
        ({value, input} = seq(d1, parser, d2)(input));

        return {
            value: value[1],
            input,
        };
    }
}

export function separatedList(parser, separator) {
    return (input) => {
        let value;

        // This is basically just the grammar for a nonempty list!
        let list = seq(
            parser,
            zeroOrMore(seq(separator, parser)),
        );

        try {
            ({value, input} = list(input));
        } catch {
            // Parse this as an empty list
            return {
                value: [],
                input,
            }
        }

        let [firstElement, subsequentElements] = value;

        // Remember that seq(separator, parser) returns an array:
        // [separatorValue, parserValue]
        // so we have to extract the second element from each result.
        subsequentElements = subsequentElements.map((list => list[1]))

        return {
            value: [firstElement, ...subsequentElements],
            input,
        };
    }
}
