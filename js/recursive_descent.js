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

// innerList ::= element {", " element} | empty
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

// list ::= "[" inner_list "]"
function list(input) {
    if (!input.startsWith("[")) {
        throw "list: expected \"[\"";
    }

    input = input.slice(1);

    let value;

    // did you know that js has destructuring assignments?
    ({input, value} = innerList(input));

    if (!input.startsWith("]")) {
        throw "list: expected \"]\"";
    }

    return {
        value,
        input: input.slice(1),
    }
}

// element ::= list | char
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

console.log(JSON.stringify(list("[a, b, [c, d], [], [[e]]]")))