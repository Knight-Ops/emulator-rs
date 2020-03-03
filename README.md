# Emulator-rs
In it's current (initial) form. This is a RV32I emulator that conforms to the compliance testing that is available for the RV32I architecture

This implementation is currently passing all compliance tests from the [riscv-tests repository](https://github.com/riscv/riscv-tests). This testing is currently hacked together to test the specific instructions using objdump, and all tests are included in the `tests/rv32i-compliance` directory. This hack is necessary because the emulator does not currently support virtual memory, and it does not follow a hardware implementation's boot sequence (requiring a reset vector and starting at a specific address).

In it's current state, the emulator is capable of running artibrary RV32I code, but essentially `NOP`s out all ECALL/EBREAK/CSR* instructions. If you do not need these instructions the emulator works according to the specification.
