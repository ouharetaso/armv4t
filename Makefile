TARGET = program.bin
AS = arm-none-eabi-as
OBJCOPY = arm-none-eabi-objcopy
ASFLAGS = -march=armv4t



$(TARGET): program.o
	$(OBJCOPY) -O binary --only-section=.text $< $@

program.o: program.s
	$(AS) $(ASFLAGS) -o $@ $<