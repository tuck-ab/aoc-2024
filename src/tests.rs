macro_rules! test_solution {
    ($($title:ident, $day:expr, $part:expr, $sol:expr)?) => {
    $(
        #[test]
        fn $title() {
            assert_eq!(crate::run($day, $part), $sol)
        }
    )*
    };
}

test_solution!(day_1_1, 1, 1, "2756096");
test_solution!(day_1_2, 1, 2, "23117829");
test_solution!(day_2_1, 2, 1, "383");
test_solution!(day_2_2, 2, 2, "436");
test_solution!(day_3_1, 3, 1, "173419328");
test_solution!(day_3_2, 3, 2, "90669332");
test_solution!(day_4_1, 4, 1, "2370");
test_solution!(day_4_2, 4, 2, "1908");
test_solution!(day_5_1, 5, 1, "4872");
test_solution!(day_5_2, 5, 2, "5564");
test_solution!(day_6_1, 6, 1, "5564");
test_solution!(day_6_2, 6, 2, "1976");
test_solution!(day_7_1, 7, 1, "7710205485870");
test_solution!(day_7_2, 7, 2, "20928985450275");
test_solution!(day_8_1, 8, 1, "240");
test_solution!(day_8_2, 8, 2, "955");
test_solution!(day_9_1, 9, 1, "6330095022244");
test_solution!(day_9_2, 9, 2, "6359491814941");
test_solution!(day_10_1, 10, 1, "461");
test_solution!(day_10_2, 10, 2, "875");
test_solution!(day_11_1, 11, 1, "199946");
test_solution!(day_11_2, 11, 2, "237994815702032");
test_solution!(day_12_1, 12, 1, "1483212");
test_solution!(day_12_2, 12, 2, "897062");