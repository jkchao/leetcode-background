


#[allow(dead_code)]
pub fn compare_version_numbers(version1: String, version2: String) -> i32 {
    let mut num1 = version1.split('.');
    let mut num2 = version2.split('.');


    let mut i = 0;
    let l1 = num1.clone().count();
    let l2 = num2.clone().count();


    while i < l1 || i < l2 {

        let n1: i32 = match num1.next() {
            Some(n) => n.parse::<i32>().unwrap(),
            None => 0,
        };

        let n2: i32 = match num2.next() {
            Some(n) => n.parse::<i32>().unwrap(),
            None => 0,
        };

        if n1 > n2 {
            return 1;
        }


        if n1 < n2 {
            return -1;
        }


        i += 1;
    }


    return 0;
}



#[test]
fn internal() {
    let res = compare_version_numbers("1.01".to_string(), "1.001".to_string());
    assert_eq!(res, 0);

    let res = compare_version_numbers("1.0".to_string(), "1.00".to_string());
    assert_eq!(res, 0);

    let res = compare_version_numbers("0.1".to_string(), "1.1".to_string());
    assert_eq!(res, -1);

    let res = compare_version_numbers("1.0.1".to_string(), "1".to_string());
    assert_eq!(res, 1);

    let res = compare_version_numbers("7.5.2.4".to_string(), "7.5.3".to_string());
    assert_eq!(res, -1);
}
