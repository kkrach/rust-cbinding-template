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

#include <netinet/in.h>

enum zoo {
    ZOO_A = AF_INET6,
    ZOO_B,
    ZOO_C,
    ZOO_D
};

void foo_reset();
enum zoo foo_get_value();
void foo_set_value(enum zoo value);

#endif // LIBFOO_FOO_H
