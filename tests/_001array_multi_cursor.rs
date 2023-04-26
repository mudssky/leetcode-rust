use leetcode_rust::_001array::multi_cursor::remove_element;

#[test]
fn test_remove_element() {
    struct TestCase {
        input: Vec<i32>,
        input2: i32,
        expect: i32,
    }
    let test_cases = &mut vec![
        TestCase {
            input: vec![3, 2, 2, 3],
            input2: 3,
            expect: 2,
        },
        TestCase {
            input: vec![0, 1, 2, 2, 3, 0, 4, 2],
            input2: 2,
            expect: 5,
        },
    ];
    for case in test_cases {
        let res = remove_element(&mut case.input, case.input2);
        assert_eq!(res, case.expect)
    }
}
