# least_8

## [Problem](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=fe53eb4c4c37d13feffe427bdac97e4e)

```Rust
// OPTIMIZE THIS FUNCTION TO RUN AS FAST AS POSSIBLE
// Result must work on play.rust-lang.org

/// Returns the 8 smallest numbers found in the supplied vector
/// in order smallest to largest.
fn least_8(l: &Vec<u32>) -> Vec<u32> {
    let mut ll = l.clone();
    ll.sort();
    ll[0..8].iter().cloned().collect()
}

// DO NOT CHANGE ANYTHING BELOW THIS LINE

fn make_list() -> Vec<u32> {
    const SIZE: usize = 1<<16;
    let mut out = Vec::with_capacity(SIZE);
    let mut num = 998_244_353_u32; // prime
    for i in 0..SIZE {
        out.push(num);
        // rotate and add to produce some pseudorandomness
        num = (((num << 1) | (num >> 31)) as u64 + (i as u64)) as u32;
    }
    return out;
}

fn main() {
    let l = make_list();
    let start = std::time::Instant::now();
    let l8 = least_8(&l);
    let end = std::time::Instant::now();
    assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    println!("Took {:?}", end.duration_since(start));
}
```

### Benchmark

Note, that `fn least_8` implements a **naive** solution with the following benchmark on my laptop:

```
test naive            ... bench:   2,003,496 ns/iter (+/- 15,684)
```

## [`fn optimized` solution](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=40797ebb912e59a9c02085751d93eb19)

### Idea 

Form the result in one pass through the original array, accumulating the smallest values in the sorted result array

### Implementation

```Rust
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
const CAPACITY: usize = 8;

```

### Benchmark

`fn optmized` solution has the following benchmark on my laptop:

```
test optimized        ... bench:     198,033 ns/iter (+/- 6,061)
```

which is 10 times faster than **naive** solution


## [`fn thread_optimized` solution](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=de69ebb784295801caa584e2cac94c8c)

### Idea 

We can use threads to parallelize the work by splitting the original array into several subarrays, finding the 8 smallest values in each of them, and merging these results at the end

### Implementation

```Rust

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

const CAPACITY: usize = 8;

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
```


### Benchmark

`fn thread_optimized` solution has the following benchmark on my laptop:

```
test thread_optimized ... bench:     107,895 ns/iter (+/- 4,352)
```

which is almost 2 times faster than `fn optimized` solution


## Summary

Thus, we managed to speed up the work of the algorithm for finding the 8 smallest values by 20 times