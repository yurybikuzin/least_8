const CAPACITY: usize = 8;

pub fn naive(l: &[u32]) -> Vec<u32> {
    let mut ll = l.to_owned();
    ll.sort();
    ll[0..CAPACITY].to_vec()
}

// form the result in one pass through the original array, accumulating the smallest values in the sorted result array
pub fn optimized(l: &[u32]) -> Vec<u32> {
    let mut ret = Vec::with_capacity(CAPACITY);
    for element in l {
        match ret.binary_search(element) {
            Ok(index) | Err(index) => {
                if ret.len() < CAPACITY {
                    ret.insert(index, *element);
                } else if index < CAPACITY {
                    ret.truncate(CAPACITY - 1);
                    ret.insert(index, *element);
                }
            }
        }
    }
    ret
}

pub fn thread_optimized(l: &[u32]) -> Vec<u32> {
    use std::thread;
    const MAX_THREADS_COUNT: usize = 4;
    let threads_count = std::cmp::min(
        MAX_THREADS_COUNT,
        l.len() / CAPACITY + if l.len() % CAPACITY == 0 { 0 } else { 1 },
    );
    let mut rets = Vec::with_capacity(threads_count);
    let l_len = l.len();
    let len = l_len / threads_count;
    let mut threads = Vec::with_capacity(threads_count);
    for i in 0..threads_count {
        let mut ret = Box::new([0u32; CAPACITY]);
        let ret_ptr: *mut u32 = (*ret).as_mut_ptr();
        let b = MyBox {
            len: if i < threads_count - 1 {
                len
            } else {
                l_len - len * i
            },
            ptr: unsafe { l.as_ptr().add(len * i) },
            ret_ptr,
        };
        rets.push(ret);
        threads.push(thread::spawn(move || unsafe {
            thread_optimized_helper(b);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let mut idxs = vec![0; threads_count];
    let mut values = Vec::with_capacity(threads_count);
    for i in 0..threads_count {
        values.push(rets[i].get(idxs[i]));
    }
    let mut ret = Vec::with_capacity(CAPACITY);
    loop {
        let mut min_value_wrapper = None;
        for (i, value) in values.iter().enumerate() {
            if let Some(value) = value {
                if let Some((_, min_value)) = min_value_wrapper {
                    if value < min_value {
                        min_value_wrapper = Some((i, value));
                    }
                } else {
                    min_value_wrapper = Some((i, value));
                }
            }
        }
        if let Some((i_of_min_value, min_value)) = min_value_wrapper {
            ret.push(**min_value);
            idxs[i_of_min_value] += 1;
            values[i_of_min_value] = rets[i_of_min_value].get(idxs[i_of_min_value]);
        }
        if ret.len() == CAPACITY {
            break;
        }
    }
    ret
}

struct MyBox {
    ptr: *const u32,
    len: usize,
    ret_ptr: *mut u32,
}
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

unsafe fn thread_optimized_helper(arg: MyBox) {
    let mut len = 0usize;
    for i in 0..arg.len {
        let element = *arg.ptr.add(i);
        let mut size = len;
        let mut left = 0;
        let mut right = size;
        let index = loop {
            if left >= right {
                break left;
            }
            let mid = left + size / 2;
            let cmp = element.cmp(&*arg.ret_ptr.add(mid));
            use std::cmp::Ordering::*;
            match cmp {
                Greater => left = mid + 1,
                Less => right = mid,
                _ => break mid,
            }
            size = right - left;
        };
        use core::ptr;
        if len < CAPACITY {
            let p = arg.ret_ptr.add(index);
            if index < len {
                ptr::copy(p, p.add(1), len - index);
            }
            ptr::write(p, element);
            len += 1;
        } else if index < CAPACITY {
            let p = arg.ret_ptr.add(index);
            if index < CAPACITY - 1 {
                ptr::copy(p, p.add(1), CAPACITY - 1 - index);
            }
            ptr::write(p, element);
        }
    }
}
