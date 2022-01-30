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

#include <string.h>

#define FOO_VALUE_DEFAULT (ZOO_A)

static enum zoo foo_value = FOO_VALUE_DEFAULT;

void foo_reset()
{
	foo_value = FOO_VALUE_DEFAULT;
}

enum zoo foo_get_value()
{
	return foo_value;
}

void foo_set_value(enum zoo value)
{
	foo_value = value;
}
