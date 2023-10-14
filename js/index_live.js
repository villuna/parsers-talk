import { satisfy, delimited, string, either, separatedList } from "./parsers.js";

// Parser goes here ...



// Html stuff so that the website works
let parser = list;

let parseButton = document.getElementById("parse");
let input = document.getElementById("input");
let output = document.getElementById("output");

parseButton.addEventListener("click", () => {
    let textInput = input.value;

    try {
        console.log(JSON.stringify(parser(textInput), null, "\t"));
        output.innerHTML = JSON.stringify(parser(textInput), null, "&nbsp;");

    } catch (error) {
        output.innerText = `error - ${error}`;
    }
});