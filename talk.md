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
