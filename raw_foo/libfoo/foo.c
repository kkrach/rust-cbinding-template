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


FOO_Error foo_read_file(const char *filename, char *buf, size_t buflen)
{
	FILE *file = NULL;
	size_t read_bytes = 0;

	file = fopen(filename, "r");
	if (NULL == file)
	{
		return FOO_FILE_NOT_FOUND;
	}
	while (!feof(file))
	{
		read_bytes += fread(buf, buflen, 1, file);
		if (ferror(file))
		{
			fclose(file);
			return FOO_ERROR_READ;
		}
	}

	fclose(file);
	return FOO_OK;
}
