import { satisfy, delimited, string, either, separatedList } from "./parsers.js";

var letter = satisfy((c) => c >= 'a' && c <= 'z');
var element = either(letter, list);

function list(input) {
    return delimited(
        string("["), 
        separatedList(element, string(", ")), 
        string("]")
    )(input);
}

console.log(JSON.stringify(list("[c, d]")));
console.log(JSON.stringify(list("[a, b, [c, d], [], [[e]]]")));