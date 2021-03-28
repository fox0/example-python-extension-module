use std::thread;

pub(crate) fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}

pub(crate) fn process() {
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let mut x = 0;
                for _ in 0..5_000_000 {
                    x += 1
                }
                x
            })
        })
        .collect();

    for h in handles {
        println!(
            "Thread finished with count={}",
            h.join().map_err(|_| "Could not join a thread!").unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::func::{process, sum_as_string};

    #[test]
    fn sum_as_string1() {
        assert_eq!(sum_as_string(1, 2), "3");
    }

    #[test]
    fn process1() {
        process();
        assert!(true);
    }
}
