# least_8

## Problem


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

[Rust playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=fe53eb4c4c37d13feffe427bdac97e4e)

### Benchmark

Note, that `fn least_8` implements a **naive** solution with the following benchmark on my laptop:

```
test naive            ... bench:   2,003,496 ns/iter (+/- 15,684)
```

## `fn optimized` solution

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

[Rust playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=40797ebb912e59a9c02085751d93eb19)

### Benchmark

`fn optmized` solution has the following benchmark on my laptop:

```
test optimized        ... bench:     198,033 ns/iter (+/- 6,061)
```

which is 10 times faster than **naive** solution


## `fn thread_optimized` solution

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

[Rust playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=de69ebb784295801caa584e2cac94c8c)

### Benchmark

`fn thread_optimized` solution has the following benchmark on my laptop:

```
test thread_optimized ... bench:     107,895 ns/iter (+/- 4,352)
```

which is almost 2 times faster than `fn optimized` solution

### `fn cheat_optimized` solution

### Idea 

Let's assume we know in advance that 8 smallest values fits 8 bits each, so we can do **cheat optimization**: will take in account only tiny (u8) values, and use u64 as accumulator of 8 smallest values

### Implementation


```Rust
pub fn cheat_optimized(l: &[u32]) -> Vec<u32> {
    type AccuType = u64;
    type AccuItemType = u8;
    const ACCU_ITEM_TYPE_BITS_LEN: usize = 8;
    // ===================
    let mut accu: AccuType = 0;
    let mut len = 0usize;
    for element in l
        .iter()
        .filter(|i| **i <= AccuItemType::MAX as u32)
        .map(|i| *i as AccuItemType)
    {
        let mut size = len;
        let mut left = 0;
        let mut right = size;
        let index = loop {
            if left >= right {
                break left;
            }
            let mid = left + size / 2;
            let mid_value = (accu >> (mid * ACCU_ITEM_TYPE_BITS_LEN)) as AccuItemType;
            let cmp = element.cmp(&mid_value);
            use std::cmp::Ordering::*;
            match cmp {
                Greater => left = mid + 1,
                Less => right = mid,
                _ => break mid,
            }
            size = right - left;
        };
        struct Mask {
            tail: AccuType,
            head: AccuType,
        }
        let mut masks = Vec::with_capacity(CAPACITY);
        let mut tail = AccuType::MAX;
        masks.push(Mask { tail, head: !tail });
        for _ in 1..CAPACITY {
            tail = tail << ACCU_ITEM_TYPE_BITS_LEN;
            masks.push(Mask { tail, head: !tail });
        }
        let (need_set, need_space) = if len < CAPACITY {
            let ret = (true, index < len);
            len += 1;
            ret
        } else if index < CAPACITY {
            (true, index < CAPACITY - 1)
        } else {
            (false, false)
        };
        if need_space {
            let mask = unsafe { masks.get_unchecked(index) };
            accu = ((accu & mask.tail) << ACCU_ITEM_TYPE_BITS_LEN) | (accu & mask.head);
        }
        if need_set {
            accu |= (element as AccuType) << (index * ACCU_ITEM_TYPE_BITS_LEN);
        }
    }
    let mut ret = Vec::with_capacity(CAPACITY);
    let mut mask = AccuItemType::MAX as AccuType;
    for i in 0..len {
        ret.push(((accu & mask) >> (i * ACCU_ITEM_TYPE_BITS_LEN)) as u32);
        mask <<= ACCU_ITEM_TYPE_BITS_LEN;
    }
    ret
}
const CAPACITY: usize = 8;
```

[Rust playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=eaecfb808492fb99545a06fcfd19ce86)

### Benchmark

`fn cheat_optimized` solution has the following benchmark on my laptop:

```
test cheat_optimized      ... bench:      15,144 ns/iter (+/- 1,187)
```

which is 133 times faster than **naive** solution

### `fn less_cheat_optimized` solution

### Idea 

Let's weaken our assumption, suppose we know in advance that 8 smallest values fits 16 bits each, so we can do **less cheat optimization**: will take in account only small (u16) values, and use **two u64 slots** (because of 64-bit architecture) as accumulator of 8 smallest values

### Implementation


```Rust
pub fn less_cheat_optimized(l: &[u32]) -> Vec<u32> {
    type AccuType = u64;
    const ACCU_TYPE_BITS_LEN: usize = 64;
    type AccuItemType = u16;
    const ACCU_ITEM_TYPE_BITS_LEN: usize = 16;
    // ===================
    type StoreInner = [AccuType; Store::LEN];
    #[derive(Clone, Copy)]
    struct Store(StoreInner);
    impl Store {
        const LEN: usize = (ACCU_ITEM_TYPE_BITS_LEN * CAPACITY) / ACCU_TYPE_BITS_LEN;
        const ITEMS_PER_SLOT: usize = ACCU_TYPE_BITS_LEN / ACCU_ITEM_TYPE_BITS_LEN;
        #[inline]
        fn new_with_same(value: AccuType) -> Self {
            Self([value; Self::LEN])
        }
        #[inline]
        fn new_with(inner: StoreInner) -> Self {
            Self(inner)
        }
        #[inline]
        fn new_head_from_tail(tail: &Self) -> Self {
            let mut inner = [0; Self::LEN];
            for i in 0..Self::LEN {
                inner[i] = !*unsafe { tail.0.get_unchecked(i) };
            }
            Store::new_with(inner)
        }
        #[inline]
        fn get(&self, i: usize) -> AccuItemType {
            let slot_idx = Self::i_to_slot_idx(i);
            (*unsafe { self.0.get_unchecked(slot_idx) }
                >> ((i - slot_idx * Self::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN))
                as AccuItemType
        }
        #[inline]
        fn i_to_slot_idx(i: usize) -> usize {
            i / Self::ITEMS_PER_SLOT
        }
    }
    // ===================
    struct Mask {
        tail: Store,
        head: Store,
    }
    impl Mask {
        fn new(tail: Store) -> Self {
            Self {
                head: Store::new_head_from_tail(&tail),
                tail,
            }
        }
    }
    let mut masks = Vec::with_capacity(CAPACITY);
    let mut tail = Store::new_with_same(AccuType::MAX);
    masks.push(Mask::new(tail));
    for i in 1..CAPACITY {
        let slot_idx = Store::i_to_slot_idx(i);
        if i % Store::ITEMS_PER_SLOT != 0 {
            *unsafe { tail.0.get_unchecked_mut(slot_idx) } <<= ACCU_ITEM_TYPE_BITS_LEN;
        }
        masks.push(Mask::new(tail));
    }
    // ===================
    let mut accu = Store::new_with_same(0);
    let mut len = 0usize;
    // ===================
    use std::cmp::Ordering::*;

    for element in l
        .iter()
        .filter(|i| **i <= AccuItemType::MAX as u32)
        .map(|i| *i as AccuItemType)
    {
        let mut size = len;
        let mut left = 0;
        let mut right = size;
        let index = loop {
            if left >= right {
                break left;
            }
            let mid = left + size / 2;
            let mid_value = accu.get(mid);
            let cmp = element.cmp(&mid_value);
            match cmp {
                Greater => left = mid + 1,
                Less => right = mid,
                _ => break mid,
            }
            size = right - left;
        };
        let (need_set, need_space) = if len < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            let ret = (Some(slot_idx), (index < len).then_some(slot_idx));
            len += 1;
            ret
        } else if index < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            (Some(slot_idx), (index < CAPACITY - 1).then_some(slot_idx))
        } else {
            (None, None)
        };
        if let Some(slot_idx) = need_space {
            let mask = unsafe { masks.get_unchecked(index) };
            for i in ((slot_idx + 1)..Store::LEN).rev() {
                *unsafe { accu.0.get_unchecked_mut(i) } = (unsafe { accu.0.get_unchecked(i) }
                    << ACCU_ITEM_TYPE_BITS_LEN)
                    | (unsafe { accu.0.get_unchecked(i - 1) }
                        >> (ACCU_TYPE_BITS_LEN - ACCU_ITEM_TYPE_BITS_LEN));
            }
            *unsafe { accu.0.get_unchecked_mut(slot_idx) } =
                ((unsafe { accu.0.get_unchecked(slot_idx) }
                    & unsafe { mask.tail.0.get_unchecked(slot_idx) })
                    << ACCU_ITEM_TYPE_BITS_LEN)
                    | (unsafe { accu.0.get_unchecked(slot_idx) }
                        & unsafe { mask.head.0.get_unchecked(slot_idx) });
        }
        if let Some(slot_idx) = need_set {
            *unsafe { accu.0.get_unchecked_mut(slot_idx) } |= (element as AccuType)
                << ((index - slot_idx * Store::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN);
        }
    }

    #[inline]
    fn accu_to_vec(accu: Store, len: usize) -> Vec<u32> {
        let mut ret = Vec::with_capacity(CAPACITY);
        let mut mask = AccuItemType::MAX as AccuType;
        for i in 0..len {
            let slot_idx = Store::i_to_slot_idx(i);
            ret.push(
                ((unsafe { accu.0.get_unchecked(slot_idx) } & mask)
                    >> ((i - slot_idx * Store::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN))
                    as u32,
            );
            if i % Store::ITEMS_PER_SLOT == Store::ITEMS_PER_SLOT - 1 {
                mask = AccuItemType::MAX as AccuType
            } else {
                mask <<= ACCU_ITEM_TYPE_BITS_LEN;
            }
        }
        ret
    }

    accu_to_vec(accu, len)
}
const CAPACITY: usize = 8;
```

[Rust playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=9265a2b031275b64043ad0d5e2233cad)

### Benchmark

`fn less_cheat_optimized` solution has the following benchmark on my laptop:

```
test less_cheat_optimized ... bench:      83,345 ns/iter (+/- 3,951)
```

which is 5 times slower than **cheat optimized** solution, but still 24 times faster than **naive** solution


### `fn non_cheat_optimized` solution

### Idea 

Let's stop cheating at all, but use **four u64 slots** (because of 64-bit architecture) as accumulator of 8 smallest u32 values

### Implementation


```Rust
pub fn non_cheat_optimized(l: &[u32]) -> Vec<u32> {
    type AccuType = u64;
    const ACCU_TYPE_BITS_LEN: usize = 64;
    type AccuItemType = u32;
    const ACCU_ITEM_TYPE_BITS_LEN: usize = 32;
    // ===================
    type StoreInner = [AccuType; Store::LEN];
    #[derive(Clone, Copy)]
    struct Store(StoreInner);
    impl Store {
        const LEN: usize = (ACCU_ITEM_TYPE_BITS_LEN * CAPACITY) / ACCU_TYPE_BITS_LEN;
        const ITEMS_PER_SLOT: usize = ACCU_TYPE_BITS_LEN / ACCU_ITEM_TYPE_BITS_LEN;
        #[inline]
        fn new_with_same(value: AccuType) -> Self {
            Self([value; 4])
        }
        #[inline]
        fn new_with(inner: StoreInner) -> Self {
            Self(inner)
        }
        #[inline]
        fn new_head_from_tail(tail: &Self) -> Self {
            let mut inner = [0; Self::LEN];
            for i in 0..Self::LEN {
                inner[i] = !*unsafe { tail.0.get_unchecked(i) };
            }
            Store::new_with(inner)
        }
        #[inline]
        fn get(&self, i: usize) -> AccuItemType {
            let slot_idx = Self::i_to_slot_idx(i);
            (*unsafe { self.0.get_unchecked(slot_idx) }
                >> ((i - slot_idx * Self::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN))
                as AccuItemType
        }
        #[inline]
        fn i_to_slot_idx(i: usize) -> usize {
            i / Self::ITEMS_PER_SLOT
        }
    }
    // ===================
    struct Mask {
        tail: Store,
        head: Store,
    }
    impl Mask {
        fn new(tail: Store) -> Self {
            Self {
                head: Store::new_head_from_tail(&tail),
                tail,
            }
        }
    }
    let mut masks = Vec::with_capacity(CAPACITY);
    let mut tail = Store::new_with_same(AccuType::MAX);
    masks.push(Mask::new(tail));
    for i in 1..CAPACITY {
        let slot_idx = Store::i_to_slot_idx(i);
        if i % Store::ITEMS_PER_SLOT != 0 {
            *unsafe { tail.0.get_unchecked_mut(slot_idx) } <<= ACCU_ITEM_TYPE_BITS_LEN;
        }
        masks.push(Mask::new(tail));
    }
    // ===================
    let mut accu = Store::new_with_same(0);
    let mut len = 0usize;
    // ===================
    use std::cmp::Ordering::*;
    for element in l.iter().copied() {
        let mut size = len;
        let mut left = 0;
        let mut right = size;
        let index = loop {
            if left >= right {
                break left;
            }
            let mid = left + size / 2;
            let mid_value = accu.get(mid);
            let cmp = element.cmp(&mid_value);
            match cmp {
                Greater => left = mid + 1,
                Less => right = mid,
                _ => break mid,
            }
            size = right - left;
        };
        let (need_set, need_space) = if len < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            let ret = (Some(slot_idx), (index < len).then_some(slot_idx));
            len += 1;
            ret
        } else if index < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            (Some(slot_idx), (index < CAPACITY - 1).then_some(slot_idx))
        } else {
            (None, None)
        };
        if let Some(slot_idx) = need_space {
            let mask = unsafe { masks.get_unchecked(index) };
            for i in ((slot_idx + 1)..Store::LEN).rev() {
                *unsafe { accu.0.get_unchecked_mut(i) } = (unsafe { accu.0.get_unchecked(i) }
                    << ACCU_ITEM_TYPE_BITS_LEN)
                    | (unsafe { accu.0.get_unchecked(i - 1) }
                        >> (ACCU_TYPE_BITS_LEN - ACCU_ITEM_TYPE_BITS_LEN));
            }
            *unsafe { accu.0.get_unchecked_mut(slot_idx) } =
                ((unsafe { accu.0.get_unchecked(slot_idx) }
                    & unsafe { mask.tail.0.get_unchecked(slot_idx) })
                    << ACCU_ITEM_TYPE_BITS_LEN)
                    | (unsafe { accu.0.get_unchecked(slot_idx) }
                        & unsafe { mask.head.0.get_unchecked(slot_idx) });
        }
        if let Some(slot_idx) = need_set {
            *unsafe { accu.0.get_unchecked_mut(slot_idx) } |= (element as AccuType)
                << ((index - slot_idx * Store::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN);
        }
    }

    #[inline]
    fn accu_to_vec(accu: Store, len: usize) -> Vec<u32> {
        let mut ret = Vec::with_capacity(CAPACITY);
        let mut mask = AccuItemType::MAX as AccuType;
        for i in 0..len {
            let slot_idx = Store::i_to_slot_idx(i);
            ret.push(
                ((unsafe { accu.0.get_unchecked(slot_idx) } & mask)
                    >> ((i - slot_idx * Store::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN))
                    as u32,
            );
            if i % Store::ITEMS_PER_SLOT == Store::ITEMS_PER_SLOT - 1 {
                mask = AccuItemType::MAX as AccuType
            } else {
                mask <<= ACCU_ITEM_TYPE_BITS_LEN;
            }
        }
        ret
    }

    accu_to_vec(accu, len)
}
const CAPACITY: usize = 8;
```

[Rust playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=90cf670acfbe3749cbbe07fbcc508ce7)

### Benchmark

`fn non_cheat_optimized` solution has the following benchmark on my laptop:

```
test non_cheat_optimized  ... bench:     284,756 ns/iter (+/- 2,982)
```

which is just 7 times faster than **naive** solution and almost 1.5 time slower than `fn optimized` and almost 3 times slower than `fn thread_optimized`

## Intermediate result

### Benchamarks

We have following implementations benchmarks:

```
test naive                ... bench:   2,007,155 ns/iter (+/- 16,965)
test non_cheat_optimized  ... bench:     284,756 ns/iter (+/- 2,982)
---
test optimized            ... bench:     197,925 ns/iter (+/- 1,057)
test thread_optimized     ... bench:     105,191 ns/iter (+/- 18,821)
test less_cheat_optimized ... bench:      46,876 ns/iter (+/- 1,018)
test cheat_optimized      ... bench:      15,079 ns/iter (+/- 224)
```

The last four appear suitable for production under the appropriate circumstances.

But wait. Maybe we can do more?

## Final enhancement

Let's use **rightmost guard**: before dealing with the next element, we compare it with rightmost selected element, the largest among the selected 8 smallest.
And if it's not less, move on to the next one.

Applying the **rightmost guard** enhancement brings us to the following picture:

```
test naive                ... bench:   2,041,462 ns/iter (+/- 7,865)
test thread_optimized     ... bench:      54,301 ns/iter (+/- 4,864)
test less_cheat_optimized ... bench:      32,869 ns/iter (+/- 1,716)
test non_cheat_optimized  ... bench:      25,829 ns/iter (+/- 679)
test optimized            ... bench:      15,376 ns/iter (+/- 359)
test cheat_optimized      ... bench:      14,747 ns/iter (+/- 353)
```

## Conclusion

In `fn optimized` We achieved almost 150x speedup of a **naive** algorithm without using threads, bitwise operations, unsafe or cheats. Just plain Rust.

Here our choice:

```Rust
pub fn optimized(l: &[u32]) -> Vec<u32> {
    let mut ret = Vec::with_capacity(CAPACITY);
    let mut right_value = 0;
    for element in l {
        if ret.len() == CAPACITY && *element >= right_value {
            continue;
        }
        match ret.binary_search(element) {
            Ok(index) | Err(index) => {
                let need_insert = if ret.len() < CAPACITY {
                    Some(false)
                } else if index < CAPACITY {
                    Some(true)
                } else {
                    None
                };
                if let Some(need_truncate) = need_insert {
                    if need_truncate {
                        ret.truncate(CAPACITY - 1);
                    }
                    ret.insert(index, *element);
                    right_value = if index == ret.len() - 1 {
                        *element
                    } else {
                        ret[ret.len() - 1]
                    };
                }
            }
        }
    }
    ret
}
```

[Rust playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=bf3bd5cb6a076160363273cf79511c1a)


