/**
 * @file foo.h
 *
 * @brief Foo library for setting and getting value
 *
 * @author Karl Krach
 *
 * @license This project is released under the MIT license.
 *
 */

#ifndef LIBFOO_FOO_H
#define LIBFOO_FOO_H


void foo_reset();
int foo_get_value();
void foo_set_value(int value);


struct bar {
    int first;
    char second;
    char *third;
};

const struct bar *foo_get_bar();
void foo_set_bar(struct bar *value);


#define LOO_A 227
#define LOO_B 0xFF00FF00

enum zoo {
    ZOO_A = 7,
    ZOO_B,
    ZOO_C,
    ZOO_D
};

enum zoo foo_get_zoo();
void foo_set_zoo(enum zoo value);

#endif // LIBFOO_FOO_H
