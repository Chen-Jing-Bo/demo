pub mod traffic_light {

    pub enum Color {
        RED,
        GREEN,
        YELLOW,
    }

    pub trait Light {
        fn show_time(&self) -> i8;
    }

    impl Light for Color {
        fn show_time(&self) -> i8 {
            match self {
                Color::RED => 12,
                Color::GREEN => 24,
                Color::YELLOW => 6,
            }
        }
    }
}

pub mod sum {
    pub fn get_sum(list: Vec<u32>) -> Option<u32> {
        let mut sum: u32 = 0;
        for e in list {
            let (r, is_overflow) = sum.overflowing_add(e);
            sum = r;
            if is_overflow == true {
                return None;
            }
        }

        Some(sum)
    }
}

pub mod shape {

    pub trait Calculate {
        fn calculate_area(&self) -> f32;
    }

    pub struct Circle {
        pub r: f32,
    }

    impl Calculate for Circle {
        fn calculate_area(&self) -> f32 {
            3.14 * (self.r * self.r)
        }
    }

    pub struct Triangle {
        pub a: f32,
        pub h: f32,
    }

    impl Calculate for Triangle {
        fn calculate_area(&self) -> f32 {
            (self.a * self.h) / 2.0
        }
    }

    pub struct Square {
        pub b: f32,
    }

    impl Calculate for Square {
        fn calculate_area(&self) -> f32 {
            self.b * self.b
        }
    }

    pub fn get_area<T: Calculate>(shape: &T) -> f32 {
        shape.calculate_area()
    }
}
