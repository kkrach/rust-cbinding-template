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

struct bar bar_value;

const struct bar *foo_get_bar()
{
	return &bar_value;
}

void foo_set_bar(struct bar *value)
{
	memcpy(&bar_value, value, sizeof(bar_value));
}

static enum zoo zoo_value = ZOO_A;

enum zoo foo_get_zoo()
{
	return zoo_value;
}

void foo_set_zoo(enum zoo value)
{
	zoo_value = value;
}
