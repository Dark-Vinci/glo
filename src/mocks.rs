trait SuperTrait {
    fn do_something(&self) -> String;
}

trait ChildTrait: SuperTrait {
    fn do_more(&self) -> u32;
}

fn melon(abc: &dyn SuperTrait) -> u32 {
    if abc.do_something() == "this is something" {
        return 0;
    }

    1
}

#[cfg(test)]
use mockall::mock;

#[cfg(test)]
mock!{
    pub Nine{}

    impl SuperTrait for Nine {
        fn do_something(&self) -> String {
            "this is something".to_string()
        }
    }

    impl ChildTrait for Nine {
        fn do_more(&self) -> u32 {0}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_trait_mock() {

        // let res = melon()
        let mut mock = MockNine::new();

        mock.expect_do_something()
            .returning(|| "this is something".to_string());

        let res = melon(&mock);

        assert_eq!(res, 0);
    }
}
