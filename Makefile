.PHONY: kernel qemu qemu-gdb user

ARCH = riscv64-unknown-elf
GDB = $(ARCH)-gdb
GDB_OPTS =
QEMU = qemu-system-riscv64
QEMU_OPTS =

kernel:
	make -C kernel build

QEMU_OPTS += -machine virt
QEMU_OPTS += -nographic
QEMU_OPTS += -bios bootloader/rustsbi-qemu.bin
QEMU_OPTS += -device loader,file=kernel/kernel.bin,addr=0x80200000

qemu:kernel
	$(QEMU) $(QEMU_OPTS)

GDB_OPTS += -ex 'target remote localhost:15234'
GDB_OPTS += -ex 'set arch riscv:rv64'
GDB_OPTS += -ex 'symbol-file kernel/target/riscv64gc-unknown-none-elf/release/kernel'

qemu-gdb:kernel
	$(QEMU) $(QEMU_OPTS) -S -gdb tcp::15234 &
	sleep 1
	$(GDB) $(GDB_OPTS)
	
user:
	make -C user build
