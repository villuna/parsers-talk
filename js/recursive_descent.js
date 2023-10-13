// letter ::= 'a' | 'b' | ... | 'z'
function letter(input) {
    if (input.length == 0) {
        throw "letter: input not long enough!";
    }

    if (input[0] >= 'a' && input[0] <= 'z') {
        return {
            value: input[0],
            input: input.slice(1),
        }
    }

    throw "letter: next char was not a letter!";
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

// element ::= list | letter
function element(input) {
    try {
        return list(input);
    } catch {}

    try {
        return letter(input);
    } catch {
        throw "element: couldn't parse as either list or char";
    }
}

//console.log(JSON.stringify(list("[a, b, [c, d], [], [[e]]]bababooey")))
//console.log(JSON.Stringify(list("[a, b")))
console.log(JSON.Stringify(list("[A, b]")))