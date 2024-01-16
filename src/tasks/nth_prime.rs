pub fn nth(n: u32) -> u32 {

    let mut idx = 0;
    let mut number = 2;

    if n == idx {
        return number;
    }

    let is_prime = |value: u32| -> bool {
        for i in 2..value {
            if value % i == 0 {
                return false;
            }
         }

        true
    };

    while idx != n {
        number += 1;

        if is_prime(number) {
            idx += 1;
        }
    }

    number
}
