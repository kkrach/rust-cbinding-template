#
# Makefile for libfoo project
#


CROSS_COMPILE ?=
CC     := $(CROSS_COMPILE)gcc
AR     := ar

SRCS   := foo.c
OBJS   := $(SRCS:.c=.o)
TARGET := libfoo.a
EXAMPLE := example

CFLAGS += -Wall -Werror



default: all
all: $(TARGET)

$(TARGET): $(OBJS)
	$(AR) rcs $@ $^

$(EXAMPLE): $(OBJS) example.o
	$(CC) -o $@ $^

clean:
	rm -rf $(TARGET) $(EXAMPLE) $(OBJS)


.PHONY: default all clean
