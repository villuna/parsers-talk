# Script structure

- Introduction and high level overview
- Description of parsers
    - Manual parsers and their downsides
    - Diversion to talk about regex
    - Regex to grammars, EBNF
    - Now we have grammars, there are many ways to turning grammars into parsers
    - Parser combinators
    - Turning grammars into parsers

# Script - Parser Combinators in n different languages

Intro - in google doc

# 1. High level overview

Today's topic is of course, parser combinators. Parser combinators are a way of creating parsers in a programmatic way, that allow us to write such parsers in a declarative fashion. We essentially just tell the computer the structure of the language we want to understand, as well as some extra code to turn strings into some data the computer can understand, and it kind of "writes the parser for us". This isn't code generation or parser generators or anything fancy either, we're going to be able to achieve this with some plain old functions.

But before I really introduce you to combinators, we have to take a short hike through some more abstract topics. Since this talk is aimed at people who know how to program but may not know any formal methods of parsing, we'll have to build combinators from the ground up.

So in the interest of keeping this talk widely accessible:

## 1.1 Introduction to parsers

[Image of computer]
This is a computer.

[Image of human]
This is a human. Like most humans, this one likes to talk to computers. But computers aren't very smart, at least they can't process language like this specimen's brain can. This is why we invented parsers, to allow computers to understand us when we talk to them.

You've almost definitely written *some* kind of parser before. A parser is usually just some program that takes in a string, usually in a format that humans can understand, analyses its structure and turns the important data into a format that the *computer* can understand. It also needs to be able to fail if the string doesn't actually match our format. This doesn't even have to be a string really, it could be any data stream, but for now let's just focus on strings.

In order to write a parser programatically, we need to have a good understanding of the structure of our text format. So before we talk about writing parsers, we have to talk about another concept:

### 1.1.1 Grammars
Grammars. Now grammars are a really important concept in parsing, and definitely a fun rabbit hole to go down.

### 1.1.(1.5) Regex
..But actually before we talk about grammars actually let's take a quick digression to talk about regex. Regex is nice. How many of you are familiar with regex?

- if yes: good, this will make things a bit easier
- if lukewarm: okay, well we'll go over an example of one quickly anyway to get our feet wet.

Regexes allow you to define the format of a string declaratively, and test if a string follows that format. If a string follows the structure defined by the regex, we say it "matches" that regex. So here's an example of a regex that matches an acceptable rendition of "chugga-chugga choo choo":

```
(chugga chugga chugga chugga)+choo choo
```

you read this like '1 or more "chugga chugga chugga chugga"s followed by one "choo choo"'. The plus means "one or more of whatever thing is immediately before the plus" Okay, that's a bit silly so let's take a look at a rule that matches a positive integer:

```
0|([1-9][0-9]*)
```

Hmm.. There's more going on here. The things in square brackets are ranges, so [1-9] means "a character between 1 and 9". The asterisk is like the plus, but matches 0 or more instead of 1 or more, and that pipe operator in the middle is the or operator - i.e., the regex matches the first thing OR the second thing. So this regex reads as "either a zero or a nonzero digit followed by any number of digits.

We see that the regex reads left to right, and matches certain rules that define the structure of a format. Looking past the... let's be honest, completely unreadable syntax, this is actually a pretty natural way to think about the structure of language. Left to right, explaining what things are allowed in order.

[Slide of the cons of regex - say that the syntax was "designed by an insufferable code golfer"]

There are some problems with regex though. Regex is not powerful enough to parse complicated structures that can't be described like this. The biggest drawback is that regexes can't handle anything recursive. For example, you can't write a regex that matches a correct json file. It's just not possible. Because json files can contain maps that contain other maps that contain more maps... How could you ever make sure that all the brackets match, for instance? So regexes aren't really powerful enough to define parsers on their own.

So this is where grammars come back in.

### 1.1.2 Grammars for real this time

Okay, back to grammars. A grammar is a lot like a regex, it defines the structure of text from left to right, but now we can give names to different sub-structures. I think the best way to understand this is by looking at a couple examples so let's do that.

Here is a grammar that defines a list, whose elements are letters or other lists. For example, something like [a, [b, c], [d, [e]], [], f].

```
list ::= '[' inner_list ']'

inner_list ::= element { ', ' element } | empty

element ::= letter | list

letter ::= 'a' | 'b' | ... | 'z'
```

Have a quick look at this and see if you can see what's going on here. We can read this in a similar fashion to the regex, but now we have these definitions on the left here - these are ways of giving names to a structure. We call them "non-terminal symbols", as they are built up with smaller parts.

This grammar says "a list is an open square bracket, then an inner list, then a close square bracket. These curly braces mean the same thing as an asterisk in regex, so this means "whatever is inside these braces can be repeated 0 or more times". An inner list, then, is a single element followed by any number of elements each preceeded by a comma and a space. Alternatively, an inner list may be empty. An element is either a letter or a list, and a letter here is just characters from a to z".

This defines the structure of our format, much like regex, except in this case we are allowed to define symbols and reuse them in other rules. We can even use these symbols recursively - notice that a list may contain any number of other lists.

TODO: Research the technical definition of grammars so you can give something more rigorous

## 1.2 Grammars into parsers

So we have a way of thinking critically about our text formats. It's definitely interesting and gives us some insight about our format, but how do we turn this into a parser? Actually, there are many ways, including parser generators that will take a grammar and automatically generate a parser program for you, but today we're going to be looking at a more manual method of parsing, which is called recursive descent.

Recursive descent is a method of parsing that revolves around functions. For each terminal symbol in our grammar, we write a function that parses that symbol - that is to say, it analyses a string and turns it into some data the computer can understand.

The functions have this kind of type signature:

```
parser(String) -> (ParsedType, String) | Error
```

Let's break this down quickly. First, the function takes in a string (no surprises there). It then parses *as much as it can* from the string (starting at the beginning), and returns it along with whatever it *didn't* parse. Of course, parsers can also fail, and if that happens we instead return an error type.

Now, you might wonder why we aren't parsing the whole string, and why we return what wasn't parsed. This allows us to chain multiple parsers together. Remember how in the definition of our grammar, we listed symbols left to right? If you want to be able to parse a grammar like this:

```
symbol ::= a b
```

First, we parse symbol a. If that succeeded, then we run parser b on the rest of the input that wasn't consumed by a. *That's* the reason we return the unconsumed input.

Let's think about an example. Imagine we had a parser that parsed integers, and we gave it the string "123". Then we should expect it to return the integer one hundred and twenty three, along with an empty string - that is to say, it consumed all the input.

What if we gave it the string "123 abc"? We should expect it to succeed with the value 123 again, and this time the unconsumed output is " abc". Also notice that even though "1" and "12" are both valid integers, this parser consumed the longest valid string from the front - "123".

And now what about "abc123"? Error. It doesn't start with a valid integer.

With this definition, we now have some direction as to how to turn a grammar into a parser. We start by writing functions for all the smaller symbols, things that can be easily handled by a manual parsing method like string functions or regex, and then we build larger and larger parsers by combining smaller ones.

Already this has a lot of advantages over manual parsing methods - it can easily handle recursive formats as recursion is one of the basic uses of functions, it allows us to reuse a lot of common code, and it's a very programmatic method of turning a grammar directly into a parser.

Now the sharp-eared among us will note that I haven't yet mentioned "parser combinators" - don't worry, they're coming. But still, I think it's time we took our first step into the realm of real programming languages and actually implement a recursive descent parser.

Well, actually I lied, we're not going to be looking at a real programming language yet.

## 2 - Javascript

Because we're going to do it in Javascript.

Javascript is a language invented in 1995 for the web browser Netscape Navigator to allow programmers to add scripts to their websites and has somehow failed upwards to become the backbone of the entire world's web infrastructure.

Surprisingly, I kinda like Javascript, and we'll come to see that it's actually not such a bad choice for building a parser (if you ignore things like "performance" and if you really can't get enough of debugging runtime errors). But I legitimately do like Javascript, just, for what it was made for, which is clientside website scripts.

Fun fact! I ran into a nasty bug in my code because apparently:

```js
list.push[element];
```

Isn't a syntax error, but it silently does nothing. `:)`

But Javascript is perfect for this because it's very simple and it'll allow us to learn the basics before we go all fancy schmancy with a more sophisticated language.

Remember a few minutes ago when I told you about this grammar that parses nested lists? Let's write a recursive descent parser to do that!

### 2.1 - A Recursive, Decent Parser.

The first question to ask is, what does our parser type look like in this language? Javascript is a very weakly typed language, so we have a lot of freedom in how to define things, but what I came up with is something like this:

```js
function parser(input/*: string*/) {
    if (success) {
        return {
            // The thing that we parsed:
            value: /* ... */,

            // The unconsumed input:
            input:  /* ... */
        };
    }

    if (error) {
        throw "error message";
    }
}
```

Javascript doesn't have tuples, it basically just has maps (in fact, arrays in javascript are also basically just maps), so let's instead us a javascript object so we can give our values nice names. Also, while we could return some error type, the standard way to handle errors in js is through exceptions, so let's go with this; it'll make our code a lot simpler. Also note that you can literally throw anything in javascript! Doesn't have to be an error type. That's pretty cool. Maybe kind of terrifying too.

Looking at our grammar, the simplest symbol is probably a character, so lets look at that. Writing a parser function for this is simple enough:

```js
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
```

We just chomp off the next character if it is a to z, and return it. If the string is empty or the next character isn't a letter, we just throw an error.
For simple parsers like this, a manual approach is perfectly fine. We can do the same thing for open and close brackets too:

```js
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
```

Next, to combine parsers we can just run the functions one after another, feeding in the unconsumed input to the next parser each time. It looks like this:

```js
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
```

Let's just examine what's happening here. This syntax here is destructuring: it's equivalent to doing this:

```js
({input, value} = openBracket(input));

// Is the same thing as

let parserResult = openBracket(input);
input = parserResult.input;
value = parserResult.value;
```

First we parse this open bracket, and ignore its value. Since we're using exceptions for our error handling, if this parser fails, the whole function fails, which is what we want (every parse has to succeed, in order). We then parse the inner list - we haven't defined this function yet, but you can probably assume what it does, which is parse an inner list and return its value. We then parse the closing bracket, and then return the value we parsed.

So that's how we parse multiple elements in a row. How about this construction, where the grammar could be one of multiple alternatives? 

```
element ::= list | letter
```

In this case, we try each parser and return the first one that succeeded. With our exception handling, this is basically just a bunch of try catch blocks. So for the `element` symbol, which could be either a list or a character, we write this:

```js
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
```

We try each parser in order, and if none of them succeed we just throw an error. Notice that we're basically ignoring the errors that we encountered when parsing - we could do better here, but our current method of error handling is very unsophisticated.

Let's go back to our innerList parser that we referenced earlier but never defined; how we parse the same thing 0 or more times?

```
innerList ::= [ nonEmptyInnerList ]
nonEmptyInnerList ::= element { ", " element}
```

Well, that's just a while loop!

```js
function commaThenElement(input) {
    if (!input.startsWith(", ")) {
        throw "commaThenElement: expected \", \"";
    }

    input = input.slice(2);
    ({input, value}) = element(input);

    return {
        value: null,
        input: input.slice(2),
    };
}

// nonEmptyInnerList ::= element {", " element}
function nonEmptyInnerList(input) {
    let value;
    ({value, input} = element(input)); 

    let res = [value];

    while (true) {
        try {
            ({input, value}) = commaThenElement(input);
            res.push(value);
        } catch {
            // If we encounter an error, we don't fail, we just stop the loop
            break;
        }
    }

    return {
        value: res,
        input
    };
}
```

I've split this up into more functions than necessary just to demonstrate the principle. We parse the same thing in a while loop until it fails, then break out of the loop.

Finally, our inner list is optional; that is to say, it could be empty. And you might be able to guess that we do this with another try-catch block:

```js
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
```

I think that's all our symbols in our grammar, so let's test it out:

```js
// ...
console.log(JSON.Stringify(list("[a, b, [c, d], [], [[e]]]bababooey")))
```

```
$ node recursive_descent.js

{"value":["a","b",["c","d"],[],[["e"]]],"input":"bababooey"}
```

And... It just works. Of course this string has extra input at the end, and so in the finished parser we'll want to check that there is no unconsumed input, but it works and it turns our string into a javascript list, with the exact structure we wanted. It also rejects anything that doesn't fit our grammar. For instance, unmatched brackets:

```js
console.log(JSON.Stringify(list("[a, b")))
```

```
$ node js/recursive_descent.js

/home/luna/Documents/parsers/js/recursive_descent.js:66
        throw "closeBracket: expected \"]\"";
        ^
closeBracket: expected "]"
(Use `node --trace-uncaught ...` to show where the exception was thrown)

```

And yeah, we even get a good error message - we expected a close bracket. What if one of our elements is not a lowercase letter?

```js
console.log(JSON.Stringify(list("[A, b]")))
```

```
$ node js/recursive_descent.js

/home/luna/Documents/parsers/js/recursive_descent.js:66
        throw "closeBracket: expected \"]\"";
        ^
closeBracket: expected "]"
```

Hm. This isn't a helpful error message, it's still telling us we expected a close bracket. There is actually a good reason for this - when we tried to parse the inner list, it would have failed to parse any elements, then concluded that the list must be empty, so then it tried to parse a close bracket. Since 'A' is not a close bracket, this is the error message we get. That's one of the limits of our somewhat unsophisticated error handling, but on the bright side we still do get an error because it didn't fit our grammar.

### 2.2 - Don't repeat yourself