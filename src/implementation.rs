const CAPACITY: usize = 8;

pub fn naive(l: &[u32]) -> Vec<u32> {
    let mut ll = l.to_owned();
    ll.sort();
    ll[0..CAPACITY].to_vec()
}

// form the result in one pass through the original array, accumulating the smallest values in the sorted result array
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

pub fn cheat_optimized(l: &[u32]) -> Vec<u32> {
    type AccuType = u64;
    type AccuItemType = u8;
    const ACCU_ITEM_TYPE_BITS_LEN: usize = 8;
    // ===================
    let mut accu: AccuType = 0;
    let mut len = 0usize;
    let mut right_value: AccuItemType = 0;
    for element in l
        .iter()
        .filter(|i| **i <= AccuItemType::MAX as u32)
        .map(|i| *i as AccuItemType)
    {
        if len == CAPACITY && element >= right_value {
            continue;
        }
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
            tail <<= ACCU_ITEM_TYPE_BITS_LEN;
            masks.push(Mask { tail, head: !tail });
        }
        let need_set = if len < CAPACITY {
            let need_space = index < len;
            len += 1;
            Some(need_space)
        } else if index < CAPACITY {
            Some(index < CAPACITY - 1)
        } else {
            None
        };
        if let Some(need_space) = need_set {
            if need_space {
                let mask = unsafe { masks.get_unchecked(index) };
                accu = ((accu & mask.tail) << ACCU_ITEM_TYPE_BITS_LEN) | (accu & mask.head);
            }
            accu |= (element as AccuType) << (index * ACCU_ITEM_TYPE_BITS_LEN);
            right_value = if index == len - 1 {
                element
            } else {
                (accu >> ((len - 1) * ACCU_ITEM_TYPE_BITS_LEN)) as AccuItemType
            };
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
            for (i, slot) in inner.iter_mut().enumerate().take(Self::LEN) {
                *slot = !*unsafe { tail.0.get_unchecked(i) };
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
    let mut right_value: AccuItemType = 0;
    for element in l
        .iter()
        .filter(|i| **i <= AccuItemType::MAX as u32)
        .map(|i| *i as AccuItemType)
    {
        if len == CAPACITY && element >= right_value {
            continue;
        }
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
        let need_set = if len < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            let need_space = index < len;
            len += 1;
            Some((slot_idx, need_space))
        } else if index < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            let need_space = index < CAPACITY - 1;
            Some((slot_idx, need_space))
        } else {
            None
        };
        if let Some((slot_idx, need_space)) = need_set {
            if need_space {
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
            *unsafe { accu.0.get_unchecked_mut(slot_idx) } |= (element as AccuType)
                << ((index - slot_idx * Store::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN);
            right_value = if index == len - 1 {
                element
            } else {
                accu.get(len - 1)
            };
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
            for (i, slot) in inner.iter_mut().enumerate().take(Self::LEN) {
                *slot = !*unsafe { tail.0.get_unchecked(i) };
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
    let mut right_value: AccuItemType = 0;
    for element in l.iter().copied() {
        if len == CAPACITY && element >= right_value {
            continue;
        }
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
        let need_set = if len < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            let need_space = index < len;
            len += 1;
            Some((slot_idx, need_space))
        } else if index < CAPACITY {
            let slot_idx = Store::i_to_slot_idx(index);
            let need_space = index < CAPACITY - 1;
            Some((slot_idx, need_space))
        } else {
            None
        };
        if let Some((slot_idx, need_space)) = need_set {
            if need_space {
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
            *unsafe { accu.0.get_unchecked_mut(slot_idx) } |= (element as AccuType)
                << ((index - slot_idx * Store::ITEMS_PER_SLOT) * ACCU_ITEM_TYPE_BITS_LEN);
            right_value = if index == len - 1 {
                element
            } else {
                accu.get(len - 1)
            };
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
        let mut min_value_wrapper: Option<(usize, u32)> = None;
        for (i, value) in values
            .iter()
            .enumerate()
            .filter_map(|(i, value)| value.map(|value| (i, value)))
        {
            let need_update = if let Some((_, min_value)) = min_value_wrapper {
                *value < min_value
            } else {
                true
            };
            if need_update {
                min_value_wrapper = Some((i, *value));
            }
        }
        if let Some((i_of_min_value, min_value)) = min_value_wrapper {
            ret.push(min_value);
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
    let mut right_value: u32 = 0;
    for i in 0..arg.len {
        let element = *arg.ptr.add(i);
        if len == CAPACITY && element >= right_value {
            continue;
        }
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
        let need_set = if len < CAPACITY {
            let need_space = (index < len).then_some(len - index);
            len += 1;
            Some(need_space)
        } else if index < CAPACITY {
            let need_space = (index < CAPACITY - 1).then_some(CAPACITY - 1 - index);
            Some(need_space)
        } else {
            None
        };
        if let Some(need_space) = need_set {
            use core::ptr;
            let p = arg.ret_ptr.add(index);
            if let Some(len) = need_space {
                ptr::copy(p, p.add(1), len);
            }
            ptr::write(p, element);
            right_value = if index == len - 1 {
                element
            } else {
                *arg.ret_ptr.add(len - 1)
            };
        }
    }
}
