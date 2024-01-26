use kernel32;
use winapi;
use winapi::MEMORY_BASIC_INFORMATION;

use winapi::{
    DWORD,                                        // In rust, a u32
    HANDLE,                                       // Pointer types for various internal APIs without an associated type. In Rust, std::os::raw::c_void defines void pointers; a HANDLE is a pointer to some opaque resource within Windows.
    LPVOID,                                       
    PVOID,                                        // In Windows, data type names are often prefixed with a shorthand for their type. P stands for pointer; LP stands for long pointer (e.g., 64 bit).
    SIZE_T,                                       // u64 = usize
    LPSYSTEM_INFO,                                // A pointer to a SYSTEM_INFO struct
    SYSTEM_INFO,                                  // Some structs defined by Windows internally
    MEMORY_BASIC_INFORMATION as MEMINFO,          // Some structs defined by Windows internally
};

fn main() {
    let this_pid: DWORD;                          // Initializes these variables from within unsafe blocks. To make these accessible in the outer scope, these need to be defined here.
    let this_proc: HANDLE;                        // 
    let min_addr: LPVOID;                         // 
    let max_addr: LPVOID;                         // 
    let mut base_addr: PVOID;                     // 
    let mut proc_info: SYSTEM_INFO;               // 
    let mut mem_info: MEMORY_BASIC_INFORMATION;   // 

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();

    unsafe {                                      // This block guarantees that all memory is initialized.
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe {                                        // This block of code is where system calls are made.
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        /*
            Rather than use a return value, this function makes
            use of a C idiom to
            provide its result to the caller. We provide a pointer
            to some predefined struct, then read that struct’s new values once the
            function returns to see the results
        */
        kernel32::GetSystemInfo(                   //
          &mut proc_info as LPSYSTEM_INFO          // 
        );                                         // 
    };

    min_addr = proc_info.lpMinimumApplicationAddress; // Renaming these variables for convenience
    max_addr = proc_info.lpMaximumApplicationAddress; // ...

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);


    loop {                                         // This loop does the work of scanning through the address space.
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(              // Provides information about a specific segment of the running program’s memory address space, starting at base_addr
                                    this_proc, base_addr,
                                    &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
