// global variables in rust: static
// static variables can only store references with the 'static lifetime
static HELLO_WORLD: &str = "Hello, world!";
const HELLO_CONST: &str = "Hello, const!";
// const HELLO_CONST: &str = "Hello, const!";

// static variables can be mutable, but it is an unsafe operation
// must use unsafe to read or write COUNTER
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Hello, world!");
    let mut num = 5;
    const HELLO_CONST: &str = "Hello, const!";

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        // println!("{}", *r);
    }
    unsafe {
        dangerous();
    }
    let address = 0x01234usize;
    let r = address as *mut i32;

    // using values will likely crash the program
    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    //

    // calling a function from another program
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3.0));
    }

    println!("name is : {}", HELLO_WORLD);
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// unsafe functions
unsafe fn dangerous() {}



use std::slice;

// safe abstraction to unsafe code
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // cannot do in safe rust, because borrow checker doesnt understand we are mutably borrowing
    // from mutually exclusive slices
    // (&mut values[..mid], &mut values[mid..])
    //
    //
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// "C" defines what ABI to use, the ABI defines how to call this function at the assembly level
extern "C" {
    fn abs(input: f32) -> f32;
}

// how to export rust functions to other languages
// disable mangling to make compiled name more readable to other languages
// does not require unsafe
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
