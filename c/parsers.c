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

typedef struct {
    parser_t parser;
    size_t result_size;
} many0_data;

typedef struct {
    void *list;
    int length;
} many0_result;

char *many0_fn(char *input, void *vdata, void *vresult) {
    many0_data *data = vdata;

    // This is a strange thing
    // we want to create a new array and write its memory address
    // to the destination pointed at by vresult 
    // if that makes any sense at all ._.
    many0_result *result = vresult;

    // If you dont know C, this is how your dynamic lists look like behind the scenes
    void *out_list = NULL;
    int length = 0;
    int capacity = 0;

    char *current_input = input;

    // We don't know the size of our output at compile time,
    // so we have to allocate it at runtime
    void *output = malloc(data->result_size);

    while (1) {
        char *unparsed = data->parser.parse(current_input, data->parser.data, output);

        if (unparsed == NULL) {
            break;
        }

        current_input = unparsed;

        // Push the new value to the vector
        if (length >= capacity) {
            // Resize the vector
            if (capacity == 0) {
                capacity = 1;
            } else {
                capacity *= 2;
            }

            out_list = realloc(out_list, data->result_size * capacity);
        }

        memcpy((void *) ((char *)out_list + length * data->result_size) , output, data->result_size);
        length++;
    }

    out_list = realloc(out_list, data->result_size * length);
    
    free(output);

    result->list = out_list;
    result->length = length;

    return current_input;
}

parser_t many0(parser_t parser, size_t result_size) {
    many0_data *data = malloc(sizeof(many0_data));
    data->parser = parser;
    data->result_size = result_size;

    parser_t result = {
        .data = data,
        .parse = many0_fn,
    };

    return result;
}

int main() {
    char* my_string = "itititititworks!";
    char r2;

    many0_result it_result;

    two_result result = {
        .one = &it_result,
        .two = &r2,
    };

    char* out = parse(two(many0(tag("it"), sizeof(slice_t)), character), my_string, &result);

    if (out != NULL) {
        printf("unconsumed output: \"%s\"\n", out);

        printf("result: ([");

        slice_t *slices = it_result.list;
        for (int i = 0; i < it_result.length; i++) {
            if (i != 0) {
                printf(", ");
            }

            printf("\"%.*s\"", slices[i].len, slices[i].string);
        }

        printf("], ");
        printf("'%c')\n", r2);
    }
}
