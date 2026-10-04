fn main() {
    // active, executive, 1100 (12), BE
    // State = 168 | 3 = 171 (0b0000_0000_0011_0011)
    let le_sys: u16 = 0b0000_0000_0011_0011;

    println!("--- Little-Endian System State ({:#018b}) ---", le_sys);
    println!("Is Active: {}", is_active(le_sys));
    println!("Is Commander: {}", is_commander(le_sys));
    println!("Is Big Endian: {}", is_big_endian(le_sys));
    println!("Squadron ID: {}\n", get_squadron_id(le_sys));

    // Example 2: Active cadet in Squadron 42, with Big-Endian flag set (Bit 9 = 1)
    // Set Bit 9 (1 << 9 = 512): State = 171 + 512 - 2 (not commander) = 681
    let be_sys: u16 = 681;

    println!("--- Big-Endian System State ({:#018b}) ---", be_sys);
    println!("Is Active: {}", is_active(be_sys));
    println!("Is Commander: {}", is_commander(be_sys));
    println!("Is Big Endian: {}", is_big_endian(be_sys));
    println!("Squadron ID: {}", get_squadron_id(be_sys));
}


// 1. System State: Create one unsigned integer variable to represent the state of your system. 
// Define the meaning of the bit flags within that variable for your specific domain 
// (e.g., bit 0 represents "is_active", bit 1 represents "is_admin", bits 2-8 represent "group_id").

// 2. Designate exactly one bit within this variable to represent the machine's endianness 
// (e.g., 0 for little-endian, 1 for big-endian). 

// 3. Write function(s) that extract specific bit(s) from the unsigned variable in order to access and 
// read your defined flags.

// 4. In one or more of your functions, read your designated endian bit to determine the format. 
// Use that bit's value to decide whether to convert a specific extracted flag into big-endian format 
// before returning it as the output.


const IS_ACTIVE_STATE: u16 = 1 << 0;     // Bit 0
const IS_EXECUTIVE_STATE: u16 = 1 << 1;  // Bit 1
const SQUADRON_ID: u16 = 0x7F << 2;   // Bits 2-8 (0b0000_0001_1111_1100)
const ENDIAN_STATE: u16 = 1 << 9;        // Bit 9


fn is_active(sys_state: u16) -> bool {
    (sys_state & IS_ACTIVE_STATE) != 0
}

/// Returns true if the session belongs to a commander (Bit 1)
fn is_commander(sys_state: u16) -> bool {
    (sys_state & IS_EXECUTIVE_STATE) != 0
}

// 0 for little-endian, 1 for big-endian
fn is_big_endian(sys_state: u16) -> bool {
    (sys_state & ENDIAN_STATE) != 0
}

fn get_squadron_id(sys_state: u16) -> u16 {
    let raw_squadron = (sys_state & SQUADRON_ID) >> 2;
    if is_big_endian(sys_state) {
        u8::from_be(raw_squadron as u8) as u16
    } else {
        raw_squadron
    }
}

// Testing: Write a testing function that takes ownership of a String. 
// This string should represent your system's core resource (e.g., a ticket ID or a user message), 
// then write another testing function that works with an i32 (a Copy type), 
// representing a system parameter like a capacity limit or a status code. 
// Observe and document the difference in how Rust handles memory for these two types.

fn print_task_name(task_name: &String) {
    // is just reference t1
    println!("Your task is: {task_name}"); 

}

fn print_sub_tasks(num_sub_tasks: i32) { 
    // creates a copy of n1 when passed as argument 
    println!("This task has {num_sub_tasks} sub tasks!"); 

    // nothing is returned!
}
