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
test_solution!(day_13_1, 13, 1, "28887");
test_solution!(day_13_2, 13, 2, "96979582619758");
test_solution!(day_14_1, 14, 1, "218433348");
test_solution!(day_14_2, 14, 2, "6512");
test_solution!(day_15_1, 15, 1, "1441031");
test_solution!(day_15_2, 15, 2, "1425169");
test_solution!(day_16_1, 16, 1, "102488");
test_solution!(day_16_2, 16, 2, "559");
test_solution!(day_17_1, 17, 1, "7,4,2,0,5,0,5,3,7");
test_solution!(day_17_2, 17, 2, "202991746427434");
test_solution!(day_18_1, 18, 1, "446");
test_solution!(day_18_2, 18, 2, "39,40");
test_solution!(day_19_1, 19, 1, "296");
test_solution!(day_19_2, 19, 2, "619970556776002");
test_solution!(day_20_1, 20, 1, "1450");
test_solution!(day_20_2, 20, 2, "1015247");
