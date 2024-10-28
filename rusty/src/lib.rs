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

fn r_index_to_perm(p: i32, r: &mut [i32]) -> () {
    if p < 0 || p >= factorial(r.len() as i32) {
        for i in 0..r.len() {
            r[i] = -1;
        }
    } else {
        // TODO: this logic flew over my head, need to revisit
        let mut p = p;
        let mut aux = vec![0; r.len()];
        for i in 0..r.len() {
            let mut c = 0;
            let mut j = 0;
            while c <= p / factorial((r.len() - i - 1) as i32) {
                if aux[j] == 0 {
                    c += 1;
                }
                j += 1;
            }
            r[i] = j as i32 - 1;
            aux[j - 1] = 1;
            p %= factorial((r.len() - i - 1) as i32);
        }
    }
}

#[no_mangle]
pub extern "C" fn index_to_perm(p: i32, n: i32, r: *mut i32) -> () {
    let r = unsafe { std::slice::from_raw_parts_mut(r, n as usize) };
    r_index_to_perm(p, r);
}

fn r_index_to_subset(s: i32, k: i32, r: &mut [i32]) -> () {
    if s < 0 || s >= binomial(r.len() as i32, k) {
        for i in 0..r.len() {
            r[i] = -1;
        }
    } else {
        // TODO: this logic flew over my head, need to revisit
        let mut s = s;
        let mut k = k;
        for i in 0..r.len() {
            if k == (r.len() - i) as i32 {
                for j in i..r.len() {
                    r[j] = 1;
                }
                return;
            } else if k == 0 {
                for j in i..r.len() {
                    r[j] = 0;
                }
                return;
            } else if s >= binomial((r.len() - i - 1) as i32, k) {
                r[i] = 1;
                k -= 1;
                s -= binomial((r.len() - i - 1) as i32, k);
            } else {
                r[i] = 0;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn index_to_subset(s: i32, n: i32, k: i32, r: *mut i32) -> () {
    let r = unsafe { std::slice::from_raw_parts_mut(r, n as usize) };
    r_index_to_subset(s, k, r);
}


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

fn r_int_to_sum_zero_array(x: i32, b: i32, a: &mut [i32]) -> () {
    // note different implementation; should be the same
    r_int_to_digit_array(x, b, a);
    if b <= 1 {
        return;
    } else {
        let mut sum = 0;
        for i in 0..(a.len() - 1) {
            sum += a[i];
        }
        a[a.len() - 1] = (b - sum) % b;
    }
}

#[no_mangle]
pub extern "C" fn int_to_sum_zero_array(x: i32, b: i32, n: i32, a: *mut i32) -> () {
    let a = unsafe { std::slice::from_raw_parts_mut(a, n as usize) };
    r_int_to_sum_zero_array(x, b, a);
}

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

fn r_perm_sign(a: &[i32]) -> i32 {
    if !r_is_perm(a) {
        return -1;
    }
    let mut result = 0;
    for i in 0..a.len() {
        for j in i+1..a.len() {
            if a[i] > a[j] {
                result += 1;
            }
        }
    }
    result % 2
}

#[no_mangle]
pub extern "C" fn perm_sign(a: *const i32, n: i32) -> i32 {
    let a = unsafe { std::slice::from_raw_parts(a, n as usize) };
    return r_perm_sign(a);
}

// int         perm_to_index(int *a, int n);
fn r_perm_to_index(a: &[i32]) -> i32 {
    if !r_is_perm(a) {
        return -1;
    }
    let mut result = 0;
    for i in 0..a.len() {
        let mut count = 0;
        for j in i+1..a.len() {
            if a[j] < a[i] {
                count += 1;
            }
        }
        result += count * factorial((a.len() - i - 1) as i32);
    }
    result
}

#[no_mangle]
pub extern "C" fn perm_to_index(a: *const i32, n: i32) -> i32 {
    let a = unsafe { std::slice::from_raw_parts(a, n as usize) };
    return r_perm_to_index(a);
}

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
