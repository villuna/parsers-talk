#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "util.h"

typedef struct {
    char* string;
    int len;
} slice_t;

typedef struct {
    void* data;
    size_t result_size;
    char* (*parse)(char* input, void* data, void* result);
} parser_t;

char* parse(parser_t parser, char* input, void* result) {
    if (!input) {
        return NULL;
    }

    return parser.parse(input, parser.data, result);
}

// character - a parser that parses a single character, whatever it is
//
// this is a simple parser that doesn't combine any other parsers
char* character_fn(char* input, void* data, void* vresult) {
    char* result = vresult;
    
    if (strlen(input) == 0)
        return NULL;

    *result = input[0];
    return input + 1;
}

parser_t character = {
    .data = NULL,
    .result_size = sizeof(char),
    .parse = character_fn,
};

// tag - a parser that parses a given string
//
// this parser needs some data supplied at runtime, but is not a combinator
char* tag_fn(char* input, void* data, void* vresult) {
    char* tag = data;
    slice_t* result = vresult;

    if (strlen(input) < strlen(tag))
        return NULL;

    for (int i = 0; i < strlen(tag); i++) {
        if (input[i] != tag[i]) {
            return NULL;
        }
    }

    slice_t out = {
        .string = input,
        .len = strlen(tag),
    };

    *result = out;

    return input + strlen(tag);
}

parser_t tag(char* tag) {
    parser_t parser = {
        .data = tag,
        .result_size = sizeof(slice_t),
        .parse = tag_fn
    };

    return parser;
}

// two - takes in two parsers and parses them in sequence.
//
// this is *almost* like >>=
// dont tell the haskell programmers i said that tho
typedef struct {
    parser_t one;
    parser_t two;
} two_data;

typedef struct {
    void* one;
    void* two;
} two_result;

char* two_fn(char* input, void* vdata, void* vresult) {
    two_data* data = vdata; 
    two_result* result = vresult;

    char* out = parse(data->one, input, result->one);
    out = parse(data->two, out, result->two);

    return out;
}

parser_t two(parser_t one, parser_t two) {
    two_data* data = malloc(sizeof(two_data));
    data->one = one;
    data->two = two;

    parser_t result = {
        .data = data,
        .parse = two_fn
    };

    return result;
}

// many0 - takes in a parser and parses it zero or more times
char* many0_fn(char* input, void* vdata, void* vresult) {
    parser_t* parser = vdata;
    vector* result = vresult;

    vector out_list = vec_new(parser->result_size);

    // We don't know the size of our output at compile time,
    // so we have to allocate it at runtime
    void* output = malloc(parser->result_size);

    while (1) {
        char* unparsed = parse(*parser, input, output);

        if (unparsed == NULL) {
            break;
        }

        vec_push(&out_list, output);
        input = unparsed;
    }
    
    free(output);

    *result = out_list;

    return input;
}

parser_t many0(parser_t parser) {
    parser_t* data = malloc(sizeof(parser_t));
    *data = parser;

    parser_t result = {
        .data = data,
        .parse = many0_fn,
    };

    return result;
}

int main() {
    char* my_string = "itititititworks!";

    char char_result;
    vector it_result;

    two_result result = {
        .one = &it_result,
        .two = &char_result,
    };

    parser_t parser = two(
        many0(tag("it")),
        character
    );

    char* out = parse(parser, my_string, &result);

    if (out != NULL) {
        printf("unconsumed output: \"%s\"\n", out);

        printf("result: ([");

        slice_t *slices = it_result.data;
        for (int i = 0; i < it_result.length; i++) {
            if (i != 0) {
                printf(", ");
            }

            printf("\"%.*s\"", slices[i].len, slices[i].string);
        }

        printf("], ");
        printf("'%c')\n", char_result);
    }

    // Clean up our dynamically allocated memory;
    free(((two_data *)parser.data)->one.data);
    free(parser.data);
    vec_free(it_result);
}
