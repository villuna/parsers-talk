#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct {
    char* string;
    int len;
} slice_t;

typedef struct {
    void* data;
    char* (*parse)(char* input, void* data, void* result);
} parser_t;

char* parse(parser_t parser, char* input, void* result) {
    if (!input) {
        return NULL;
    }

    return parser.parse(input, parser.data, result);
}

// A simple parser that doesn't combine any other parsers
char* character_fn(char* input, void* data, void* vresult) {
    char* result = vresult;
    
    if (strlen(input) == 0)
        return NULL;

    *result = input[0];
    return input + 1;
}

parser_t character = {
    .data = NULL,
    .parse = character_fn,
};

// A more complicated parser that takes in some data but doesnt combine any parsers
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
        .parse = tag_fn
    };

    return parser;
}

// A parser that does something with a parser
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

int main() {
    char* my_string = "it works!";
    slice_t r1;
    char r2;

    two_result result = {
        .one = &r1,
        .two = &r2,
    };

    char* out = parse(two(tag("it"), character), my_string, &result);

    if (out) {
        printf("output: (\"%.*s\", '%c', \"%s\")\n", r1.len, r1.string, r2, out);
    } else {
        printf("failed!\n");
    }
    
}
