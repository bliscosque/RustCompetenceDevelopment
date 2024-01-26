//This program tries to scan the memory but crashes with
//error: process didn't exit successfully: `target\debug\memscan1.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
//this is because we try to access a NULL Pointer (within unsafe) AND because we try to access memory not entitled to my program (segmentation fault)

fn main() {
    let mut n_nonzero = 0;

    for i in 1..10000 {
        // Converts i to a *const T, a raw pointer of type u8 to inspect raw memory addresses. We treat every address as a unit, ignoring the fact that most values span multiple bytes.
        let ptr = i as *const u8;   
        // Dereferences the pointer, it reads the value at address i. Another way of saying this is “read the value being pointed to.”                     
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}
