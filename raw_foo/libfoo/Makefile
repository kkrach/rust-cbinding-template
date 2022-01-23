#
# Makefile for libfoo project
#


CROSS_COMPILE ?= /opt/crosstool-ng-arm/bin/arm-unknown-linux-gnueabi-
CC     := $(CROSS_COMPILE)gcc
AR     := ar

SRCS   := foo.c
OBJS   := $(SRCS:.c=.o)
TARGET := libfoo.a

CFLAGS += -Wall -Werror



default: all
all: $(TARGET)

$(TARGET): $(OBJS)
	$(AR) rcs $@ $^

clean:
	rm -rf $(TARGET) $(OBJS)


.PHONY: default all clean
