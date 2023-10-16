// integer ::= '0' | onenine { digit }
function integer(input) {
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

// nonEmptyInnerList ::= element {", " element}
function nonEmptyInnerList(input) {
    let value;
    ({value, input} = element(input)); 

    let res = [value];

    while (true) {
        if (!input.startsWith(", ")) {
            break;
        }
        input = input.slice(2);

        ({value, input} = element(input));
        res.push(value);
    }

    return {
        value: res,
        input
    };
}

// innerList ::= [ nonEmptyInnerList ]
function innerList(input) {
    try {
        return nonEmptyInnerList(input);    
    } catch {
        // If that failed, just parse this as nothing
        return {
            value: [],
            input,
        };
    }
}

function openBracket(input) {
    if (!input.startsWith("[")) {
        throw "openBracket: expected \"[\"";
    }

    return {
        value: null,
        input: input.slice(1),
    };
}

function closeBracket(input) {
    if (!input.startsWith("]")) {
        throw "closeBracket: expected \"]\"";
    }

    return {
        value: null,
        input: input.slice(1),
    };
}

// list ::= "[" inner_list "]"
function list(input) {
    let value;

    ({input, value} = openBracket(input));

    ({input, value} = innerList(input));
    // we only care about the value of the list so 
    // lets save its result to use later
    let result = value;

    ({input, value} = closeBracket(input));

    return {
        value: result,
        input,
    }
}

// element ::= list | integer
function element(input) {
    try {
        return list(input);
    } catch {}

    try {
        return integer(input);
    } catch {
        throw "element: couldn't parse as either list or char";
    }
}

console.log(JSON.stringify(list("[1, 2, [3, 4], [[727]]]")))