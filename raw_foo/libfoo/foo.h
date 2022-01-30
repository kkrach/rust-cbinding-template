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

#include <stddef.h>

typedef enum {
	FOO_OK,
	FOO_FILE_NOT_FOUND,
	FOO_ERROR_READ,
} FOO_Error;

FOO_Error foo_read_file(const char *filename, char *buf, size_t buflen);

#endif // LIBFOO_FOO_H
