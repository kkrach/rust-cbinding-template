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
#include <stdio.h>


FOO_Error foo_read_file(const char *filename, char *buf, size_t *buflen)
{
	FILE *file = NULL;
	size_t read_bytes = 0;

	if (!filename || !buf || !buflen || !*buflen)
	{
		return FOO_INVALID_ARGUMENT;
	}

	file = fopen(filename, "r");
	if (NULL == file)
	{
		*buflen = 0;
		return FOO_FILE_NOT_FOUND;
	}
	while (!feof(file) && read_bytes < *buflen)
	{
		read_bytes += fread(buf+read_bytes, 1, *buflen-read_bytes, file);
		if (ferror(file))
		{
			*buflen = 0;
			fclose(file);
			return FOO_ERROR_READ;
		}
	}
	if (read_bytes < *buflen)
	{
		*buflen = read_bytes;
	}

	fclose(file);
	return FOO_OK;
}
