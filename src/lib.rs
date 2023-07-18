pub mod implementation;

pub fn make_list() -> Vec<u32> {
    const SIZE: usize = 1 << 16;
    let mut out = Vec::with_capacity(SIZE);
    let mut num = 998_244_353_u32; // prime
    for i in 0..SIZE {
        out.push(num);
        // rotate and add to produce some pseudorandomness
        num = (((num << 1) | (num >> 31)) as u64 + (i as u64)) as u32;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive() {
        let l = make_list();
        let l8 = implementation::naive(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn optimized() {
        let l = make_list();
        let l8 = implementation::optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn optimized_edge_case() {
        let mut l = make_list();
        l.push(38);
        let l8 = implementation::optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn optimized_edge_case2() {
        let mut l = make_list();
        l.push(37);
        let l8 = implementation::optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 37], l8);
    }

    #[test]
    fn thread_optimized() {
        let l = make_list();
        let l8 = implementation::thread_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn thread_optimized_edge_case() {
        let mut l = make_list();
        l.push(38);
        let l8 = implementation::thread_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn thread_optimized_edge_case2() {
        let mut l = make_list();
        l.push(37);
        let l8 = implementation::thread_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 37], l8);
    }

    #[test]
    fn cheat_optimized() {
        let l = make_list();
        let l8 = implementation::cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn cheat_optimized_edge_case() {
        let mut l = make_list();
        l.push(38);
        let l8 = implementation::cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn cheat_optimized_edge_case2() {
        let mut l = make_list();
        l.push(37);
        let l8 = implementation::cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 37], l8);
    }

    #[test]
    fn less_cheat_optimized() {
        let l = make_list();
        let l8 = implementation::less_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn less_cheat_optimized_edge_case() {
        let mut l = make_list();
        l.push(38);
        let l8 = implementation::less_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn less_cheat_optimized_edge_case2() {
        let mut l = make_list();
        l.push(37);
        let l8 = implementation::less_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 37], l8);
    }

    #[test]
    fn non_cheat_optimized() {
        let l = make_list();
        let l8 = implementation::non_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn non_cheat_optimized_edge_case() {
        let mut l = make_list();
        l.push(38);
        let l8 = implementation::non_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    }

    #[test]
    fn non_cheat_optimized_edge_case2() {
        let mut l = make_list();
        l.push(37);
        let l8 = implementation::non_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 37], l8);
    }
}
