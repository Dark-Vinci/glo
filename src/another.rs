

// Define the traits
trait SuperTrait {
    fn super_method(&self) -> u32;
}

trait MyTrait: SuperTrait {
    fn my_method(&self) -> u32;
}

// Conditionally compile mocks for tests
#[cfg(test)]
use mockall::automock;

#[cfg(test)]
#[automock]
trait SuperTrait {
    fn super_method(&self) -> u32;
}

#[cfg(test)]
#[automock]
trait MyTrait: SuperTrait {
    fn my_method(&self) -> u32;
}

// Production implementation
struct RealStruct;

impl SuperTrait for RealStruct {
    fn super_method(&self) -> u32 {
        42
    }
}

impl MyTrait for RealStruct {
    fn my_method(&self) -> u32 {
        24
    }
}

// Function using the trait
fn use_trait(obj: &impl MyTrait) -> u32 {
    obj.super_method() + obj.my_method()
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_use_trait() {
        let mut mock = MockMyTrait::new();

        // Mock the super trait method
        mock.expect_super_method()
            .returning(|| 10);

        // Mock the main trait method
        mock.expect_my_method()
            .returning(|| 20);

        // Test the function
        assert_eq!(use_trait(&mock), 30);
    }
}