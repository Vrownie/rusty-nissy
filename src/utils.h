#ifndef UTILS_H
#define UTILS_H

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "rusty_utils.h"

#define POW2TO6             64ULL
#define POW2TO11            2048ULL
#define POW2TO12            4096ULL
#define POW3TO7             2187ULL
#define POW3TO8             6561ULL
#define FACTORIAL4          24ULL
#define FACTORIAL6          720ULL
#define FACTORIAL7          5040ULL
#define FACTORIAL8          40320ULL
#define FACTORIAL12         479001600ULL
#define BINOM12ON4          495ULL
#define BINOM8ON4           70ULL
#define MIN(a,b)            (((a) < (b)) ? (a) : (b))
#define MAX(a,b)            (((a) > (b)) ? (a) : (b))

#endif
