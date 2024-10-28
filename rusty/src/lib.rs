// utils.c rewritten in rust. 
// r_ prefix indicate rust implementation that needs a C wrapper

fn r_apply_permutation(perm: &[i32], set: &mut [i32]) -> () {
    // TODO: could be optimized
    if !r_is_perm(perm) {
        return;
    }
    let mut aux = vec![0; set.len()];
    for i in 0..set.len() {
        aux[i] = set[perm[i] as usize];
    }
    for i in 0..set.len() {
        set[i] = aux[i];
    }
}

#[no_mangle]
pub extern "C" fn apply_permutation(perm: *const i32, set: *mut i32, n: i32) -> () {
    let perm = unsafe { std::slice::from_raw_parts(perm, n as usize) };
    let set = unsafe { std::slice::from_raw_parts_mut(set, n as usize) };
    r_apply_permutation(perm, set);
}

#[no_mangle]
pub extern "C" fn binomial(n: i32, k: i32) -> i32 {
    if n < 0 || k < 0 || k > n {
        0
    } else {
        factorial(n) / (factorial(k) * factorial(n - k))
    }
}

fn r_digit_array_to_int(a: &[i32], b: i32) -> i32 {
    a.iter().fold(
        0, 
        |acc, &x| acc * b + x
    )
}

#[no_mangle]
pub extern "C" fn digit_array_to_int(a: *const i32, n: i32, b: i32) -> i32 {
    // TODO: error handling, maaaaybe
    let a = unsafe { std::slice::from_raw_parts(a, n as usize) };
    return r_digit_array_to_int(a, b);
}

#[no_mangle]
pub extern "C" fn factorial(n: i32) -> i32 {
    if n < 0 {
        0
    } else {
        (1..=n).product()
    }
} 

// void        index_to_perm(int p, int n, int *r);
// void        index_to_subset(int s, int n, int k, int *r);

fn r_int_to_digit_array(a: i32, b: i32, r: &mut [i32]) -> () {
    if b <= 1 {
        for i in 0..r.len() {
            r[i] = 0;
        }
    } else {
        let mut a = a;
        for i in (0..r.len()).rev() {
            r[i] = a % b;
            a /= b;
        }
    }
}

#[no_mangle]
pub extern "C" fn int_to_digit_array(a: i32, b: i32, n: i32, r: *mut i32) -> () {
    let r = unsafe { std::slice::from_raw_parts_mut(r, n as usize) };
    r_int_to_digit_array(a, b, r);
}

// void        int_to_sum_zero_array(int x, int b, int n, int *a);

#[no_mangle]
pub extern "C" fn invert_digits(a: i32, b: i32, n: i32) -> i32 {
    let mut r = vec![0; n as usize];
    r_int_to_digit_array(a, b, &mut r);
    for i in 0..r.len() {
        r[i] = (b - r[i]) % b;
    }
    r_digit_array_to_int(&r, b)
}

fn r_is_perm(a: &[i32]) -> bool {
    // slight difference as original; should be the same
    let mut aux = vec![0; a.len()];
    for &x in a {
        if x < 0 || x >= a.len() as i32 || aux[x as usize] != 0 {
            return false;
        }
        aux[x as usize] = 1;
    }
    true
}

#[no_mangle]
pub extern "C" fn is_perm(a: *const i32, n: i32) -> bool {
    let a = unsafe { std::slice::from_raw_parts(a, n as usize) };
    return r_is_perm(a);
}

fn r_is_subset(a: &[i32], k: i32) -> bool {
    a.iter().filter(
        |&&x| x != 0
    ).count() == k as usize
}

#[no_mangle]
pub extern "C" fn is_subset(a: *const i32, n: i32, k: i32) -> bool {
    let a = unsafe { std::slice::from_raw_parts(a, n as usize) };
    return r_is_subset(a, k);
}

// int         perm_sign(int *a, int n);
// int         perm_to_index(int *a, int n);

#[no_mangle]
pub extern "C" fn powint(a: i32, b: i32) -> i32 {
    // direct implementation; should be the same
    if b < 0 {
        0
    } else {
        a.pow(b as u32)
    }
}

fn r_subset_to_index(a: &[i32], k: i32) -> i32 {
    if !r_is_subset(a, k) {
        binomial(a.len() as i32, k)
    } else {
        let mut k = k;
        let mut result = 0;
        for i in 0..a.len() {
            if k == (a.len() - i) as i32 {
                return result;
            } else if a[i] != 0 {
                result += binomial((a.len() - i - 1) as i32, k);
                k -= 1;
            }
        }
        result
    }
}

#[no_mangle]
pub extern "C" fn subset_to_index(a: *const i32, n: i32, k: i32) -> i32 {
    let a = unsafe { std::slice::from_raw_parts(a, n as usize) };
    return r_subset_to_index(a, k);
}

fn r_sum_arrays_mod(src: &[i32], dst: &mut [i32], m: i32) -> () {
    for i in 0..dst.len() {
        if m <= 0 {
            dst[i] = 0;
        } else {
            dst[i] = (dst[i] + src[i]) % m;
        }
    }
}

#[no_mangle]
pub extern "C" fn sum_arrays_mod(src: *const i32, dst: *mut i32, n: i32, m: i32) -> () {
    // TODO: error handling, maaaaybe
    let src = unsafe { std::slice::from_raw_parts(src, n as usize) };
    let dst = unsafe { std::slice::from_raw_parts_mut(dst, n as usize) };
    r_sum_arrays_mod(src, dst, m);
}

// probably not needed when fully implemented in rust? 

#[no_mangle]
pub extern "C" fn swap(a: &mut i32, b: &mut i32) -> () {
    std::mem::swap(a, b);
}

#[no_mangle]
pub extern "C" fn swapu64(a: &mut u64, b: &mut u64) -> () {
    std::mem::swap(a, b);
}
