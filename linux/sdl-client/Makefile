CC = gcc
CFLAGS = -lSDL2

all: main

main: main.c
	$(CC) main.c -o main $(CFLAGS)

run: all
	./main

clean:
	rm -f main