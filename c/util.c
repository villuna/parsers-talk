#include <string.h>
#include <stdlib.h>
#include "util.h"

vector vec_new(size_t element_size) {
    vector res = {
        .data = NULL,
        .element_size = element_size,
        .length = 0,
        .capacity = 0,
    };

    return res;
}

void vec_push(vector* v, void* element) {
    if (v->length == v->capacity) {
        if (v->capacity == 0) {
            v->capacity++;
        } else {
            v->capacity *= 2;
        }

        v->data = realloc(v->data, v->capacity * v->element_size);
    }

    memcpy((void *) ((char *) v->data + v->length * v->element_size), element, v->element_size);
    v->length++;
}

void vec_free(vector v) {
    free(v.data);
}
