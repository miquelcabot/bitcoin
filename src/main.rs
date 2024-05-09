mod field_element;
// mod field_element_point;
mod point;

use field_element::FieldElement;
//use field_element_point::FieldElementPoint;
use point::Point;

fn main() {
    let a = FieldElement::new(15, 31);
    let b = FieldElement::new(5, 31);
    println!("{}", a);
    println!("{}", b);
    println!("{}", a == b);
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a.pow(4));
    println!("{}", a / b);

    let p1 = Point::new(Some(2), Some(5), 5, 7).unwrap();
    let p2 = Point::new(Some(2), Some(-5), 5, 7).unwrap();
    let p3 = Point::new(Some(3), Some(7), 5, 7).unwrap();
    println!("{}", p1);
    println!("{}", p2);
    println!("{}", p3);
    println!("{}", p1.add(&p2).unwrap());
    println!("{}", p2.add(&p3).unwrap());
    /*
    let prime = 223;
    let a = FieldElement::new(0, prime).unwrap();
    let b = FieldElement::new(7, prime).unwrap();
    let valid_points = vec![
        (
            FieldElement::new(192, prime).unwrap(),
            FieldElement::new(105, prime).unwrap(),
        ),
        (
            FieldElement::new(17, prime).unwrap(),
            FieldElement::new(56, prime).unwrap(),
        ),
        (
            FieldElement::new(1, prime).unwrap(),
            FieldElement::new(193, prime).unwrap(),
        ),
    ];
    let invalid_points = vec![
        (
            FieldElement::new(200, prime).unwrap(),
            FieldElement::new(119, prime).unwrap(),
        ),
        (
            FieldElement::new(42, prime).unwrap(),
            FieldElement::new(99, prime).unwrap(),
        ),
    ];
    let a = FieldElementPoint::new(
        Some(FieldElement::new(192, prime).unwrap()),
        Some(FieldElement::new(105, prime).unwrap()),
        a.clone(),
        b.clone(),
    );

    println!("{:?}", a);

    let p4 = FieldElementPoint::new(Some(2), Some(5), 5, 7, 31).unwrap();
    let p5 = FieldElementPoint::new(Some(2), Some(-5), 5, 7, 31).unwrap();
    let p6 = FieldElementPoint::new(Some(3), Some(7), 5, 7, 31).unwrap();
    println!("{}", p4);
    println!("{}", p5);
    println!("{}", p6); */
}

/*
#[cfg(test)]
mod ecc_tests {
    use super::*;

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();


        let valid_points = vec![(192, 105), (17, 56), (1, 193)];
        let invalid_points = vec![(200, 119), (42, 99)];

        for (x_raw, y_raw) in valid_points {
            let x = FieldElement::new(x_raw, prime).unwrap();
            let y = FieldElement::new(y_raw, prime).unwrap();
            let z = Point::new(Some(x.num), Some(y.num), a.num, b.num);
            println!("{:?}", z);
            // assert!(Point::new(Some(x.num), Some(y.num), a.num, b.num).is_ok());
        }
/*
        for (x_raw, y_raw) in invalid_points {
            let x = FieldElement::new(x_raw, prime).unwrap();
            let y = FieldElement::new(y_raw, prime).unwrap();
            assert!(Point::new(Some(x.num), Some(y.num), a.num, b.num).is_err());
        }*/
    }
/*
    #[test]
    fn test_add() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let additions = vec![
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];

        for (x1_raw, y1_raw, x2_raw, y2_raw, x3_raw, y3_raw) in additions {
            let x1 = FieldElement::new(x1_raw, prime).unwrap();
            let y1 = FieldElement::new(y1_raw, prime).unwrap();
            let p1 = Point::new(Some(x1.num), Some(y1.num), a.num, b.num).unwrap();

            let x2 = FieldElement::new(x2_raw, prime).unwrap();
            let y2 = FieldElement::new(y2_raw, prime).unwrap();
            let p2 = Point::new(Some(x2.num), Some(y2.num), a.num, b.num).unwrap();

            let x3 = FieldElement::new(x3_raw, prime).unwrap();
            let y3 = FieldElement::new(y3_raw, prime).unwrap();
            let p3 = Point::new(Some(x3.num), Some(y3.num), a.num, b.num).unwrap();

            assert_eq!(p1.add(&p2).unwrap(), p3);
        }
    }

    #[test]
    fn test_rmul() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let multiplications = vec![
            (2, 192, 105, Some(49), Some(71)),
            (2, 143, 98, Some(64), Some(168)),
            (2, 47, 71, Some(36), Some(111)),
            (4, 47, 71, Some(194), Some(51)),
            (8, 47, 71, Some(116), Some(55)),
            (21, 47, 71, None, None),
        ];

        for (s, x1_raw, y1_raw, x2_raw, y2_raw) in multiplications {
            let x1 = FieldElement::new(x1_raw, prime).unwrap();
            let y1 = FieldElement::new(y1_raw, prime).unwrap();
            let p1 = Point::new(Some(x1.num), Some(y1.num), a.num, b.num).unwrap();

            let p2 = match (x2_raw, y2_raw) {
                (Some(x2), Some(y2)) => {
                    let x2 = FieldElement::new(x2, prime).unwrap();
                    let y2 = FieldElement::new(y2, prime).unwrap();
                    Point::new(Some(x2.num), Some(y2.num), a.num, b.num).unwrap()
                }
                _ => Point::new(None, None, a.num, b.num).unwrap(),
            };

            let mut result = p1.clone();
            for _ in 1..s {
                result = result.add(&p1).unwrap();
            }

            assert_eq!(result, p2);
        }
    }*/
}
*/
