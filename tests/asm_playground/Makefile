all:
	ca65 -I common -o test.o basic.s
	ld65 -C nes.cfg test.o -o test.nes
	
	GDK_BACKEND=x11 nestopia test.nes
