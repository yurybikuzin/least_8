use least_8::{implementation, make_list};

fn main() {
    let l = make_list();
    //
    let start = std::time::Instant::now();
    let l8 = implementation::naive(&l);
    let end = std::time::Instant::now();
    assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    println!("naive solution took {:?}", end.duration_since(start));

    let start = std::time::Instant::now();
    let l8 = implementation::optimized(&l);
    let end = std::time::Instant::now();
    assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    println!("optimized solution took {:?}", end.duration_since(start));

    let start = std::time::Instant::now();
    let l8 = implementation::thread_optimized(&l);
    let end = std::time::Instant::now();
    assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    println!(
        "thread_optimized solution took {:?}",
        end.duration_since(start)
    );

    let start = std::time::Instant::now();
    let l8 = implementation::cheat_optimized(&l);
    let end = std::time::Instant::now();
    assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    println!(
        "cheat_optimized solution took {:?}",
        end.duration_since(start)
    );

    let start = std::time::Instant::now();
    let l8 = implementation::less_cheat_optimized(&l);
    let end = std::time::Instant::now();
    assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    println!(
        "less_cheat_optimized solution took {:?}",
        end.duration_since(start)
    );

    let start = std::time::Instant::now();
    let l8 = implementation::non_cheat_optimized(&l);
    let end = std::time::Instant::now();
    assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    println!(
        "non_cheat_optimized solution took {:?}",
        end.duration_since(start)
    );
}
