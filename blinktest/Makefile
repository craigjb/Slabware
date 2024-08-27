default:
	riscv64-linux-gnu-as -march=rv32i -o blink.out blink.s
	riscv64-linux-gnu-objcopy -O binary blink.out blink.bin

clean:
	rm -f blink.out
	rm -f blink.bin
