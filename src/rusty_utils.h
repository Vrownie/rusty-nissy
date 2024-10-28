#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


void apply_permutation(const int32_t *perm, int32_t *set, int32_t n);

int32_t binomial(int32_t n, int32_t k);

int32_t digit_array_to_int(const int32_t *a, int32_t n, int32_t b);

int32_t factorial(int32_t n);

void index_to_perm(int32_t p, int32_t n, int32_t *r);

void index_to_subset(int32_t s, int32_t n, int32_t k, int32_t *r);

void int_to_digit_array(int32_t a, int32_t b, int32_t n, int32_t *r);

void int_to_sum_zero_array(int32_t x, int32_t b, int32_t n, int32_t *a);

int32_t invert_digits(int32_t a, int32_t b, int32_t n);

bool is_perm(const int32_t *a, int32_t n);

bool is_subset(const int32_t *a, int32_t n, int32_t k);

int32_t perm_sign(const int32_t *a, int32_t n);

int32_t perm_to_index(const int32_t *a, int32_t n);

int32_t powint(int32_t a, int32_t b);

int32_t subset_to_index(const int32_t *a, int32_t n, int32_t k);

void sum_arrays_mod(const int32_t *src, int32_t *dst, int32_t n, int32_t m);

void swap(int32_t *a, int32_t *b);

void swapu64(uint64_t *a, uint64_t *b);
