# Makefile for MAL Tensor C Library (OpenBLAS Optimized)
CC = gcc
CFLAGS = -O3 -Wall -Wextra -fPIC -I.
AR = ar
ARFLAGS = rcs
LDFLAGS = -lopenblas
TARGET_LIB = libmal_tensor.a
OBJS = mal_tensor.o
all: $(TARGET_LIB)
$(TARGET_LIB): $(OBJS)
	$(AR) $(ARFLAGS) $@ $^
mal_tensor.o: mal_tensor.c mal_tensor.h
	$(CC) $(CFLAGS) -c $< -o $@
# قاعدة لتنظيف الملفات المبنية
clean:
	rm -f $(OBJS) $(TARGET_LIB)
.PHONY: all clean
