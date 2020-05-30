use std::ops::Add;

pub struct Triangle<T>([T; 3]);

impl<T> Triangle<T>
where
    T: Default + Copy + PartialEq + PartialOrd + Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        if Triangle::sides_are_valid(a, b, c) {
            Some(Triangle(sides))
        } else {
            None
        }
    }

    fn sides_are_valid(a: T, b: T, c: T) -> bool {
        Triangle::sides_bigger_than_0(a, b, c) && Triangle::sides_size_correct(a, b, c)
    }

    fn sides_bigger_than_0(a: T, b: T, c: T) -> bool {
        let zero = T::default();
        a > zero && b > zero && c > zero
    }

    fn sides_size_correct(a: T, b: T, c: T) -> bool {
        a + b >= c && b + c >= a && a + c >= b
    }

    pub fn is_equilateral(&self) -> bool {
        let [first, second, third] = self.0;
        first == second && second == third
    }

    pub fn is_scalene(&self) -> bool {
        let [first, second, third] = self.0;
        first != second && first != third && second != third
    }

    pub fn is_isosceles(&self) -> bool {
        let [first, second, third] = self.0;
        first == second || second == third || first == third
    }
}
