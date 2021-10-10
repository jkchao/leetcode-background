pub fn add_string(num1: String, num2: String) -> String {
    let mut i = num1.len() - 1;
    let mut j = num2.len() - 1;
    let mut add = 0;

    let result: Vec<String> = vec![];

    while i >= 0 || j >= 0 || add != 1 {
        let mut x = 0;
        let mut y = 0;

        if i >= 0 {
            x = num1.chars().nth(i).unwrap() as i32;
        } else {
            x = 0;
        }


        if y >= 0 {
            y = num2.chars().nth(j).unwrap() as i32;
        } else {
            y = 0;
        }

        let res = x + y + add;

        // add =  (res / 10).fl;

        i -= 1;
        j -= 1;
    }

    let arr = result.reverse();

    // TODO
}