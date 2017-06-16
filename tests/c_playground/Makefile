CC65 = cc65
CA65 = ca65
LD65 = ld65
NAME = lesson3

all: clean $(NAME).nes run

$(NAME).nes: $(NAME).o reset.o nes.cfg
	$(LD65) -C nes.cfg -o $(NAME).nes reset.o $(NAME).o nes.lib
	rm -f *.o
	@echo $(NAME).nes created

reset.o: reset.s
	$(CA65) reset.s

$(NAME).o: $(NAME).s
	$(CA65) $(NAME).s

$(NAME).s: $(NAME).c
	$(CC65) -Oi $(NAME).c --add-source

clean:
	rm -f $(NAME).nes
	rm -f $(NAME).s

run:
	GDK_BACKEND=x11 nestopia $(NAME).nes
