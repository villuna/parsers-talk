If you're reading this, don't pay attention to the section numbers. They don't make sense but that's okay.

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

## 1.1 Introduction to parsers

You've almost definitely written *some* kind of parser before. A parser is usually just some program that takes in a string, usually in a format that humans can understand, analyses its structure and turns the important data into a format that the *computer* can understand. It also needs to be able to fail if the string doesn't actually match our format. This doesn't even have to be a string really, it could be any data stream, but for now let's just focus on strings.

In order to write a parser programatically, we need to have a good understanding of the structure of our text format. So before we talk about writing parsers, we have to talk about another concept:

### 1.1.1 Grammars

Grammars. Now grammars are a really important concept in parsing, and definitely a bit of a rabbit hole.

### 1.1.(1.5) Regex

..But actually before we talk about grammars actually let's take a quick digression to talk about regex. Regex is nice. How many of you are familiar with regex?

- if yes: good, this will make things a bit easier
- if lukewarm: okay, well we'll go over an example of one quickly anyway to get our feet wet.

Regexes allow you to define the format of a string declaratively, and test if a string follows that format. If a string follows the structure defined by the regex, we say it "matches" that regex. So here's an example of a regex that matches an acceptable rendition of "chugga-chugga choo choo":

```text
(chugga chugga chugga chugga)+choo choo
```

you read this like '1 or more "chugga chugga chugga chugga"s followed by one "choo choo"'. The plus means "one or more of whatever thing is immediately before the plus" Okay, that's a bit silly so let's take a look at a rule that matches a positive integer:

```text
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

```text
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

```text
parser(String) -> (ParsedType, String) | Error
```

Let's break this down quickly. First, the function takes in a string (no surprises there). It then parses *as much as it can* from the string (starting at the beginning), and returns it along with whatever it *didn't* parse. Of course, parsers can also fail, and if that happens we instead return an error type.

Now, you might wonder why we aren't parsing the whole string, and why we return what wasn't parsed. This allows us to chain multiple parsers together. Remember how in the definition of our grammar, we listed symbols left to right? If you want to be able to parse a grammar like this:

```text
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

### 2.1 - A Recursive, Decent Parser

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
// letter ::= "a" | "b" | ... | "z"
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

We just chop off the next character if it is a to z, and return it. If the string is empty or the next character isn't a letter, we just throw an error.
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

```text
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

```text
innerList ::= [ element { ", " element} ]
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

```text
$ node recursive_descent.js

{"value":["a","b",["c","d"],[],[["e"]]],"input":"bababooey"}
```

And... It just works. Of course this string has extra input at the end, and so in the finished parser we'll want to check that there is no unconsumed input, but it works and it turns our string into a javascript list, with the exact structure we wanted. It also rejects anything that doesn't fit our grammar. For instance, unmatched brackets:

```js
console.log(JSON.Stringify(list("[a, b")));
```

```text
$ node js/recursive_descent.js

/home/luna/Documents/parsers/js/recursive_descent.js:66
        throw "closeBracket: expected \"]\"";
        ^
closeBracket: expected "]"
```

And yeah, we even get a good error message - we expected a close bracket. What if one of our elements is not a lowercase letter?

```js
console.log(JSON.Stringify(list("[A, b]")));
```

```text
$ node js/recursive_descent.js

/home/luna/Documents/parsers/js/recursive_descent.js:66
        throw "closeBracket: expected \"]\"";
        ^
closeBracket: expected "]"
```

Hm. This isn't a helpful error message, it's still telling us we expected a close bracket. There is actually a good reason for this - when we tried to parse the inner list, it would have failed to parse any elements, then concluded that the list must be empty, so then it tried to parse a close bracket. Since 'A' is not a close bracket, this is the error message we get. That's one of the limits of our somewhat unsophisticated error handling, but on the bright side we still do get an error because it didn't fit our grammar.

### 2.2 - Don't repeat yourself

So that's the basics of recursive descent. And it is decent - but there are still a few problems. Mainly, particularly for simple grammars like this, it's a lot of writing. Our parser for this simple recursive list totalled to around 105 lines of code. So it's time to introduce another technique, which is parser *combinators*. Hey, we finally got to the topic of the talk!!!

Recall that a parser is just a function, of a certain type. Well, a **combinator** is a function that takes in parsers and turns them into a new parser. I.e; it is a function that takes in functions as input, and returns a new function as output. That can be strange to get our head around at first, but let's look at some examples.

Let's say I have a function called `zeroOrMore`. As the name suggests, it takes in a parser function and returns a new parser function that parses the original function zero or more times, returning a list of results. That is to say, if `letter` is a parser that parses a letter from a to z, then:

```text
zeroOrMore(letter)
```

is a parser that parses zero or more letters from a to z, and returns them all in a list. Here's an example of how that behaviour might work:

```text
> letter("abc123")
{ value: "a", input: "bc123" }

> zeroOrMore(letter)("abc")
{ value: ["a", "b", "c"], input: ""}

> letter("123")
error

> zeroOrMore(letter)("123")
{ value: [], input: "123" }
```

This syntax might seem a bit strange, but just see it as a parser that takes in an extra argument. If we do it this way, we can treat `zeroOrMore(letter)` as its own parser, and then if we want, we can pass that into other combinators, and so on.

1. My first combinator - `zeroOrMore`
Let's look at the implementation of `zeroOrMore`, to see what's going on here:

```js
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
```

The important part here is this

```js
return (input) => { ... }
```

syntax here. `(arg) => { body }` is how you declare an anonymous function in javascript. This is how you create a new function at runtime. You might have seen these before in other languages, for example in python they're called "lambdas", and in many other languages they're called "closures". All this is doing is creating a new function that takes in input and then runs the parser as many times as it can until it fails, accumulating its results in a list.

Okay, now we get the general gist of what a combinator is, let's go through a few useful ones.

### 2.3 - Combinator roundup

2. Sequence

Here's another basic one - sequence. This takes in *list* of parsers, and parses each one in a row, accumulating their results in a list. Unlike zeroOrMore, this combinator will fail if any one of the parsers in the list fails. Here's how it works

```text
> letter("ab")
{ value: "a", input: "b" }

> integer("12")
{ value: 12, input: "" }

> seq(letter, integer)("a12")
{ value: ["a", 12], input: ""}

> seq(letter, integer)("ab12")
error - not all parsers succeeded
```

The way you read this parser is that it parses a letter, then an integer. So that's what it will return - a letter, and then an integer. If that's not the format of our string, it will fail.

This is the implementation:

```js
export function seq(...parsers) {
    return (input) => {
        let value;
        let res = [];

        for (let parser of parsers) {
            ({value, input} = parser(input));
            res.push(value);
        }

        return {
            value: res,
            input,
        };
    }
}
```

Those three dots in the input argument just mean that we can pass in any number of arguments into the function and they'll all be collected into a list. It just means instead of writing `seq([letter, integer])` we write `seq(letter, integer)`, but it does the same thing.

3. Either

Another thing we had in our grammar was this alternative construction

```text
symbol ::= a | b | c | ...
```

I.e., our symbol is either a or b or c, and so on. `either` is a combinator that does just that - it takes in a list of combinators and tries each of them in sequence, and just returns the result of the first one that succeeds:

```js
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
```

And here's an example of its behaviour:

```text
> either(letter, integer)("a1")
{ value: "a", input: "1" }

> either(letter, integer)("1a")
{ value: 1, input: "a" }

> either(letter, integer)("?????")
error: either: none of the parsers succeeded
```

4. Even more powerful combinators - separated list

Combinators can also be as complicated as you want them to be! Here's one that matches a separated list. It takes in two parsers, one of them for the element and one for the separator, and it matches this grammar:

```text
separatedList ::= [ element {separator element} ]
```

This is the same as our grammar for lists that we saw earlier, but in this case it's generalised to be any separator and any element. Here is the implementation:

```js
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
```

This code is maybe a bit more complicated - it's okay, you don't need to know exactly what it's doing, but notice that we start by constructing a new parser which is an element followed by zero or more of the separator followed by another element. This is basically just our grammar. Then if that fails we return an empty list, otherwise we return the list of elements we parsed.

Let's have a look at it's behaviour to see what it's doing:

```text
> comma(" ,hello")
{ value: ", ", input: "hello" }

> separatedList(integer, comma)("1")
{ value: [1], input: "" }

> separatedList(integer, comma)("1, 26, 1337")
{ value: [1, 26, 1337], input: ""}

> separatedList(integer, comma)("should parse nothing")
{ value: [], input: "should parse nothing" }
```

We can now create parsers that parse separated lists in exactly one function call. It's also quite readable too; it is a "list of integers, separated by commas". That's very declarative and makes sense to our human brains, but the surprising part is that it *also makes sense to the computer*.

### 2.4 - All together now

Okay, I think we have almost enough to write our recursive list parser again. It's time for LIVE CODING.

## 3 - RUST MENTIONED!!!

Okay, this talk isn't called "parser combinators in javascript" so let's move onto our next language, which is rust. Now rust is my favourite language, and it has a few requirements that javascript doesn't have. Rust is statically typed, so the types of all of our parsers have to be specified at compile time. We also handle errors a bit differently, as we'll come to see.

We already know how combinators work so let's instead analyse a pre-existing combinator library. This is how you would actually do combinatorial parsing by the way, just pull in a library instead of reinventing the wheel.

The library we're looking at is called nom (I think that's a pretty cute name - especially bc of how these parsers work :3). As you might expect, nom mostly consists of a bunch of convenience functions. There's a markdown document on github that tries to list them all, let's have a look at that: https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

[Talk a little bit about the combinators]

These functions have a similar type to what we're already familiar with, but slightly different as rust doesn't have error handling. The definition of a parser type in nom is a little complicated so here's a simplified version:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

type Parser<Input, Output, Error> = fn(Input) -> Result<(Input, Output), Error>;
```

These types here are generic parameters - we're saying that they can be literally anything. Also I've written out their full names like this, but from now on we'll just be using a single capital letter for generic types:

```rust
type Parser<I, O, E> = fn(I) -> Result<(I, O), E>
```

So how do we read this? This Result type is a standard type in Rust, it's how you do error handling. It is a type that can be one of two variants - either it is Ok, and contains a result value, or it is Err and it contains an error value.

So our parser is a function from some generic type I (it doesn't have to be a string), to either an Ok variant containing our unconsumed input and our output value, or an Err variant containing an error value.

Generic types are a bit hard to reason about so here's an example for a function that parses a 32-bit integer from a string:

```rust
fn integer(input: &str) -> Result<(&str, i32), IntegerParseError>;
```

And notice now we can define our own types for error handling. This gives us a very expressive error handling system. It also allows us to use any input type we want. In fact, nom was originally designed just to parse binary formats, streams of bytes; but it was extended to handle pretty much anything including utf-8 strings.

Okay, it's time to dive into documentation! https://docs.rs/nom/latest/nom/index.html

If I have time, I would like to write a simple .ini parser with you, but if I have no time I'll just explore a prewritten one or maybe my toy programming language interpreter.

## 4. Loose ends and honourary mentions

Alright. It's been a bit of a journey, and we're basically done for the informative portion of the talk. But given that parsing is such a big topic, There are many more things I would have loved to talk about but can't because of scope and time so here are a few pointers if you would like to learn more.

### 4.1 - Functional Programming

What we have done today is a style of parsing that was pioneered in the scope of functional programming languages. None of the languages we're using today are functional programming languages - in fact, javascript is a non-functional programming language - but this technique definitely is a functional style.

- Little spiel about what functional programming is
- There are tradeoffs for writing this style in non-fp languages, for instance you get to write imperative code but you don't get the nice things like monads or out-of-order evaluation
- Would you like to know more? Take COMP3400

### 4.2 - Programming Languages

Writing parsers for structured text formats like this is pretty cool, but there are a few other techniques which come in handy when you're parsing complicated formats like programming languages.

A programming language parser usually contains two parts. The first part is called a "lexer", which turns text into a stream of what we call "tokens" - those are just the basic symbols that make up the language. This lexer strips away unnecessary things like whitespace or comments, and might do some basic parsing of integers and the like.

So for example, this simple rust program:

```rust
if i == 1 {
    print("hello");
}
```

Gets put through the lexer and turned into a stream of tokens that look something like this:

```rust
[If, Identifier("i"), Eq, Integer(1), LeftCurly, Identifier("print"), LeftBracket, String("hello"), RightBracket, Semicolon, RightCurly]
```

This is a linear stream of tokens with no structure whatsoever, but makes the grammar for the language comparatively easier to write because we don't need to worry about things like whitespace and the like.

The second parser would take this format and parse it into what's called an Abstract Syntax Tree. This is outside of our scope, this is more to do with compilers than parsing, but if you would like to know more there are myriad resources online, and of course you can take the brilliant course COMP4403 (Compilers and Interpreters), which goes into way more detail about all this stuff.

So we would basically define two functions:

```rust
fn lexer(input: &str) -> IResult<&str, TokenStream, LexError> {
    // ...
}

fn parser(input: TokenStream) -> IResult<TokenStream, AbstractSyntaxTree, ParseError> {
    // ...
}
```

And then we'd do something with the output.

### 4.3 - There are *more* methods of parsing???

Finally, we've been talking about recursive descent, but that's not the only method of parsing that exists. There are soo many more, so let's go through a quick "who would win". So without further ado - go team combinators!!

#### 4.3.1 - Combinators vs Manual parsing

The obvious alternative to a formal method of parsing is no formal method of parsing. This is a perfectly fine way of doing things for many applications, and it's probably what you're most used to. When I say "manual parsing" what I mean is using basic string manipulation functions.

Pros of manual parsing:
- Extremely simple. Not much code is needed.
- Usually the most performant as the compiler is more has an easier time optimising it than combinator recursion hell
- No libraries required

Cons:
- As soon as the format gets slightly complicated (or worse - recursive), manual parsers become impossible to understand and ridden with bugs.
- Often the format becomes dictated by the parser rather than the other way round (the format is like this because it's easy to parse)
- Takes a LONG TIME to write. Small things like handling whitespace become hell.

Similarities?:
- Both are "just code", we don't need some extra program to generate a parser for us

So manual parsing is fine when you can easily write the parser in your head, like a comma separated list of integers, for example. But what if you wanted to extend that and add *strings*. Now the strings can contain commas, so you can't just split on commas like you did when the list just contained integers. So now you have to rewrite the ENTIRE THING from scratch. So manual parsing is basically untennable for complicated formats.

#### 4.3.2 - Combinators vs Generators

Another, easier method of creating a parser from a grammar is to use a parser generator, like yacc or GNU bison. These are programs that take in a specification of a grammar and just create a parser for you. There are many generators out there, there's one for rust called Pest, in COMP4403 you use one called JavaCUP.

Pros of Generators:
- Don't have to write any code whatsoever
- Generally quite reliable (if you use them right - google cloudbleed)
- Errors are handled well without having to do a lot of thinking
- Probably your best choice for complicated formats unless you like writing a lot of code

Cons:
- These generators often just parse into a tree of strings, so you'll have to do some extra work to convert to whatever datatype you need
- Generated code is annoying to read and maintain
- Big generators, big dependencies; overkill for an uncomplicated format
- The grammar format is another language you have to learn

#### 4.3.3 - Combinators vs Regex

This is a lot like the manual parsing part, but I figured I'd mention regex.

I love regex, but I reckon you shouldn't use regex for parsing. In my testing, using regex to extract things out of a string is actually slower than using combinators, not including the time it takes to compile the regex. It's also harder to read and maintain.

HOWEVER. Regex is very fast and very good at just *recognising* if a string satisfies a certain format. It's also great for things like finding parts of a string that match a certain format without having to parse the entire string. So regex is really good for some things, and worse at others.

#### 4.3.4 - Combinators vs, y'know, just like JSON or whatever

Go for it. There are many reasons you would want to use a pre-existing filetype. Parsers are fun but usually unnecessary.

---

Okay. I hope this has given you a lot to think about, and hopefully now you're ready to go home and find a combinator library in your favourite language and write a parser for something fun. Here are some ideas:

- Invent your own configuration language and parse it (creative!)
- A parser that analyses logical expressions and prints out their truth table (mathematical!!)
- Your own dialect of the lisp programming language ((((((((Impressive)))))))!!!)
- A parser that recognises the rules and starting state of a turing machine and (turing complete!!!!)

But before we wrap up, I'd like to just show one more language.

## 5 - The C Programming Language, by Brian Kernighan, Dennis Ritchie and Luna Borella 

This is how how I took the square peg of combinators and forced them into the round hole of the C programming language.

Because this is the last part of the talk and I don't have time I'm going to assume a basic knowledge of C. So first of all, what's our parser type? We can't return tuples, so instead I landed on this, which is a reasonable way to do multiple return in C:

```c
char* parser(char* input, void* output);
```

We basically just write our return type to the output pointer. I've made it void star, this is how we do polymorphism in C, we basically just ignore the type and pass around memory addresses.

But hold on. Anonymous functions. Big problem right? We can't create an anonymous function at runtime, we can only define functions at compile time (footnote: you can in GCC but it's not standard). So we can't make combinators, because there's no way to make a define a function that takes in parsers and returns a new parser.

Or is there?

Here is my awful solution:

```c
typedef struct {
    void* data;
    char* (*parse)(char* input, void* output, void* data);
} parser_t;
```

So, what this means is that a parser contains
1. some runtime-dependent data that is passed into the parser when it is called,
2. a function that takes in our input and our runtime data and parses a string like usual

I don't think this is going to make sense without an example, so it's time to dive into the code.

## 6 - Conclusion

Um, in conclusion. Parser combinators are great, and you can implement them in your favourite language with a bit of ingenuity and maybe a little bit of madness. Go have fun with them.

Thank you for coming to my talk?