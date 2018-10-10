//! Layout engine for rhombi

use super::LayoutGenerator;

pub struct Rhombus;

enum Gen {
    Index,
    Bound,
}

impl Rhombus {
    #[inline]
    fn gen(index: u64, sw: Gen) -> u64 {
        (0..index)
            .filter_map(|i| {
                let b = 2 * i.pow(2) + 2 * i + 1;
                if index <= b {
                    Some(match &sw {
                        Gen::Index => (i + 1),
                        Gen::Bound => b,
                    } as u64)
                } else {
                    None
                }
            }).next()
            .unwrap()
    }

    /// Get x location of an index
    #[inline]
    fn x(gen: u64, index: u64) -> i64 {
        if index == 1 {
            return 0;
        }

        // Index range for current generation
        let prev = Self::generation_bound(gen - 1);
        let curr = Self::generation_bound(gen);
        let len = curr - prev;

        // Tracking where in generation we are
        let mut seek = prev + 1;

        // Offset to current generation walk
        let mut ox = (gen - 1) as i64;
        let mut down = true;

        for i in 0..=len {
            if seek == index {
                return ox;
            }
            seek += 1;

            if ox == -((gen - 1) as i64) {
                down = false;
            }

            ox += if down { -1 } else { 1 };
        }

        unreachable!()
    }

    /// Get y location of an index
    #[inline]
    fn y(index: u64) -> i64 {
        match index {
            0 => 0,
            _ => 0,
        }
    }

    /// Calculate generational bound
    fn generation_bound(index: u64) -> u64 {
        Self::gen(index, Gen::Bound)
    }
    /// Calculate generational index
    fn generation(index: u64) -> u64 {
        Self::gen(index, Gen::Index)
    }
}

#[test]
fn generations() {
    let one = Rhombus::generation_bound(1);
    assert_eq!(one, 1);

    let two = Rhombus::generation_bound(2);
    assert_eq!(two, 5);

    let three = Rhombus::generation_bound(6);
    assert_eq!(three, 13);
}

impl LayoutGenerator for Rhombus {
    fn rel_position(index: u64) -> (i64, i64) {
        let upper = Self::generation_bound(index);
        let generation = Self::generation(index);

        let foo = String::new();

        println!("Index: {}", index);
        let x = Self::x(generation, index);

        (x, 0)
    }
}
