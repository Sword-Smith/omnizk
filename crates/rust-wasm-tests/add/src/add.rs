use c2zk_stdlib::*;

#[inline(never)]
#[no_mangle]
fn add_function(a: u64, b: u64) -> u64 {
    a + b
}

#[no_mangle]
pub fn main_add() {
    let a = pub_input();
    let b = pub_input();
    let r = add_function(a, b);
    let c = secret_input();
    let r2 = add_function(r, c);
    pub_output(r2);
}
