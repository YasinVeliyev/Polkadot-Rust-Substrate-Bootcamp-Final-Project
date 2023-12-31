// struct FilterCondition<T: PartialOrd + PartialEq + Copy> {
//     filter: T,
// }

// impl<T: PartialOrd + PartialEq + Copy> FilterCondition<T> {
//     fn new(filter: T) -> Self {
//         Self { filter }
//     }

//     fn is_match(&self, other: T) -> bool {
//         self.filter == other
//     }

//     fn custom_filter(&self, v: Vec<T>) -> Vec<T> {
//         v.iter()
//             .filter(|a| self.is_match(**a))
//             .map(|a| *a)
//             .collect()
//     }
// }

struct FilterCondition {
    filter: i32,
}

impl FilterCondition {
    fn new(filter: i32) -> Self {
        Self { filter }
    }

    fn is_match(&self, other: i32) -> bool {
        other % self.filter == 0
    }
}

fn custom_filter(v: Vec<i32>, filter: &FilterCondition) -> Vec<i32> {
    v.into_iter().filter(|a| filter.is_match(*a)).collect()
}
fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 3, 8, 3, 9];
    let filter = FilterCondition::new(3);
    let new_vec = custom_filter(vector, &filter);
    println!("{:?}", new_vec);
}
