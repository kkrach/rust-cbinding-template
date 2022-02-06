#
# Makefile for libfoo project
#


CROSS_COMPILE ?= /opt/crosstool-ng-arm/bin/arm-unknown-linux-gnueabi-
CC     := $(CROSS_COMPILE)gcc
AR     := ar

SRCS   := foo.c
OBJS   := $(SRCS:.c=.o)
TARGET := libfoo.so

CFLAGS += -Wall -Werror



default: all
all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) --shared -o $@ $^

clean:
	rm -rf $(TARGET) $(OBJS)


.PHONY: default all clean
