import { satisfy, delimited, string, either, separatedList } from "./parsers.js";

// Parser goes here

console.log(JSON.stringify(list("[c, d]")));
console.log(JSON.stringify(list("[a, b, [c, d], [], [[e]]]")));