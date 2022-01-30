
#include <stdio.h>
#include "foo.h"

int main(int argc, char** argv)
{
	FOO_Error result;

	if (argc != 2)
	{
		printf("Error: invalid parameter count!\n");
		printf("\n");
		printf("Usage: %s INPUT_FILE\n", argv[0]);
		return 1;
	}

	const char *filename = argv[1];
	char buf[1024];
	size_t len = sizeof(buf);

	result = foo_read_file(filename, buf, &len);
	if (FOO_OK != result)
	{
		printf("Error: Failed to read '%s'!\n", filename);
		return 7;
	}
	printf("Read %zd bytes\n", len);
	return 0;
}
