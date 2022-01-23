/**
 * @file foo.c
 *
 * @brief Foo library for setting and getting value
 *
 * @author Karl Krach
 *
 * @license This project is released under the MIT license.
 *
 */

#include "foo.h"

#define FOO_VALUE_DEFAULT (42)

static int foo_value = FOO_VALUE_DEFAULT;

void foo_reset()
{
	foo_value = FOO_VALUE_DEFAULT;
}

int foo_get_value()
{
	return foo_value;
}

void foo_set_value(int value)
{
	foo_value = value;
}
