#ifndef UTIL_H
#define UTIL_H

#include <stdlib.h>

typedef struct {
    void* data;
    int length;
    int capacity;
    size_t element_size;
} vector;

vector vec_new(size_t element_size);
void vec_push(vector* v, void* element);
void vec_free(vector v);

#endif
