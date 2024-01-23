const REGION_SCOPE: &str = "TEST_REGION_SCOPE";

pub struct Regional<T> {
    single_region: T,
    multi_region: T,
}

impl<T> Regional<T> {
    pub fn new(single_region: T, multi_region: T) -> Self {
        Self {
            single_region,
            multi_region,
        }
    }
}

impl<T> AsRef<T> for Regional<T> {
    fn as_ref(&self) -> &T {
        match std::env::var(REGION_SCOPE)
            .unwrap_or("single-region".into())
            .as_str()
        {
            "multi-region" => &self.multi_region,
            "single-region" | _ => &self.single_region,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Regional, REGION_SCOPE};

    #[derive(PartialEq, Debug, Clone)]
    struct Output {
        value: i64,
    }

    #[test]
    fn test_regional_scoped() {
        let single_region = Output { value: 42 };
        let multi_region = Output { value: 33 };
        let out = Regional::new(single_region.clone(), multi_region.clone());

        std::env::set_var(REGION_SCOPE, "single-region");
        assert_eq! {
            out.as_ref(),
            &single_region,
        }

        std::env::set_var(REGION_SCOPE, "multi-region");
        assert_eq! {
            out.as_ref(),
            &multi_region,
        }
    }
}
