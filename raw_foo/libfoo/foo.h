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

#endif // LIBFOO_FOO_H
