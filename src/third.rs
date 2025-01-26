#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
trait SuperTrait {
    fn do_something(&self) -> String;
}

#[cfg(test)]
impl SuperTrait for MockChildTrait {
    fn do_something(&self) -> String {
        self.expect_do_something()
            .call()
            .unwrap_or_else(|| "default".to_string())
    }
}

#[cfg_attr(test, automock)]
pub trait ChildTrait: SuperTrait {
    fn do_more(&self) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_trait_mock() {

        let mut mock = MockChildTrait::new();

        mock.do_something();

        mock.expect_do_more()
            .returning(|| 42);

        // Test `do_something` (from `SuperTrait`)
        assert_eq!(mock.do_something(), "mocked");

        // Test `do_more` (from `ChildTrait`)
        assert_eq!(mock.do_more(), 42);
    }
}
