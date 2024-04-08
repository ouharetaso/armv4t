TARGET = program.bin
CC = arm-none-eabi-gcc
OBJCOPY = arm-none-eabi-objcopy
CFLAGS = -march=armv4t -O0



$(TARGET): program.o
	$(OBJCOPY) -O binary --only-section=.text $< $@

program.o: program.s
	$(CC) -c $(CFLAGS) -o $@ $<

.PHONY: clean
clean:
	rm -f *.o *.bin