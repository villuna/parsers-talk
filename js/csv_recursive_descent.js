// A basic recursive descent parser for csv files,
// without the use of combinators
//
// It's pretty good, but it doesnt handle errors or whitespace very well.
// This is a function of my laziness - to write a rec-dec
// parser that does things "correctly" we would need to do a lot
// of thinking similar to regular manual parsing. and I hate thinking!!!
// We're also repeating ourselves a lot and doing a lot of reinventing the wheel.
// we can do better!

// csv ::= {line}
function csv(input) {
    let value;
    let res = [];

    while (input.length > 0) {
        ({input, value} = line(input));

        res.push(value);
    }

    return res;
}

// line ::= int {, int} ["\n"] | 
function line(input) {
    let res = []
    let value;

    try {
        ({input, value} = int(input));
        res.push(value);
    } catch {
        return {
            input: input,
            value: [],
        };
    }

    while (true) {
        if (!input.startsWith(",")) {
            break;
        }

        ({input, value} = int(input.slice(1)));
        res.push(value);
    }

    if (input.startsWith("\n")) {
        input = input.slice(1);
    }

    return {
        input: input,
        value: res,
    };
}

function int(input) {
    let length = 0;

    while (length < input.length && input[length] >= '0' && input[length] <= '9') {
        length++;
    }

    if (length == 0) {
        throw "int: no digits";
    }

    return {
        value: Number(input.slice(0, length)),
        input: input.slice(length),
    };
}

console.log(JSON.stringify(csv("1,2,3\n4,5,6\n")));