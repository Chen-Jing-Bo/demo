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
