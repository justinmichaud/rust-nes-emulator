CA65 = ca65
LD65 = ld65

all: clean test.nes run

test.nes: test.o nes.cfg
	$(LD65) -C nes.cfg -o test.nes test.o nes.lib
	rm -f *.o

test.o: test.s
	$(CA65) test.s

clean:
	rm -f test.nes

run:
	GDK_BACKEND=x11 nestopia test.nes
