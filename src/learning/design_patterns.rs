/// # Creational patterns
///
/// ## Abstract Factory
///
/// > **Abstract Factory** is a creational design pattern that lets you produce families of related objects without specifying their concrete classes.
///
/// ### Problem
///
/// Imagine that you’re creating a furniture shop simulator. Your code consists of classes that represent:
/// + A family of related products, say: `Chair` + `Sofa` + `CoffeeTable`.
/// + Several styles of this family. For example, products `Chair` + `Sofa` + `CoffeeTable` are available in these variants: `Modern`, `Victorian`, `ArtDeco`.
///
/// If you want the make a product line for each class, you will have way too many lines.
///
/// ### Solution
/// Abstract Factory pattern suggests to find things in common, in this case, despite what exactly they called, they are no more then `Chair`, `Sofa` and `Coffee Table`,
/// so we only have to make product line for those 3 classes.The next move is to declare the Abstract Factory—an interface with a list of creation methods for all
/// products that are part of the product family (for example, createChair, createSofa and createCoffeeTable). These methods must return abstract product types represented by the interfaces we extracted previously: Chair, Sofa, CoffeeTable and so on.
mod abstract_factory {
    // Abstracted Chair
    pub trait Chair {
        fn has_legs(&self) -> bool;
        fn sit_on(&self) -> bool;
    }

    // Abstracted Coffee Table
    pub trait CoffeeTable {
        fn has_legs(&self) -> bool;
        fn has_coffees(&self) -> u32;
    }

    // Abstracted Sofa
    pub trait Sofa {
        fn has_back(&self) -> bool;
        fn sits(&self) -> u32;
    }

    // Abstracted Furniture Factory
    pub trait FurnitureFactory {
        type Chair: Chair;
        type CoffeeTable: CoffeeTable;
        type Sofa: Sofa;
        fn create_chair(&self) -> Self::Chair;
        fn create_coffee_table(&self) -> Self::CoffeeTable;
        fn create_sofa(&self) -> Self::Sofa;
    }

    // Concrete Chair variant 1
    pub struct VictorianChair {
        occupied: bool,
        legs: u32,
    }

    // Concrete Chair variant 2
    pub struct ModernChair {
        occupied: bool,
    }

    // Concrete Coffee Table variant 1
    pub struct VictorianCoffeeTable {
        coffees: u32,
    }

    //
    pub struct ModernCoffeeTable {
        coffees: u32,
    }

    pub struct VictorianSofa {
        sits: u32,
    }

    pub struct ModernSofa {
        sits: u32,
    }

    //
    impl Chair for VictorianChair {
        fn has_legs(&self) -> bool {
            println!("VictorianChair has legs");
            true
        }

        fn sit_on(&self) -> bool {
            println!("VictorianChair is currently {}", self.occupied);
            self.occupied
        }
    }

    impl VictorianChair {
        fn legs(&self) -> u32 {
            println!("VictorianChair has {} legs", self.legs);
            self.legs
        }
    }

    impl Chair for ModernChair {
        fn has_legs(&self) -> bool {
            println!("ModernChair has legs");
            true
        }

        fn sit_on(&self) -> bool {
            println!(
                "ModernChair is now {}",
                match self.occupied {
                    true => "Occupied",
                    _ => "Vacant",
                }
            );
            self.occupied
        }
    }

    impl CoffeeTable for VictorianCoffeeTable {
        fn has_legs(&self) -> bool {
            println!("Victorian CoffeeTable has legs");
            true
        }

        fn has_coffees(&self) -> u32 {
            println!(
                "Victorian CoffeeTable currently has {} coffees",
                self.coffees
            );
            self.coffees
        }
    }

    impl CoffeeTable for ModernCoffeeTable {
        fn has_legs(&self) -> bool {
            println!("Modern CoffeeTable has legs");
            true
        }

        fn has_coffees(&self) -> u32 {
            println!("Modern CoffeeTable now has {} coffee(s)", self.coffees);
            self.coffees
        }
    }

    impl Sofa for VictorianSofa {
        fn has_back(&self) -> bool {
            println!("Victorian Sofa has back");
            true
        }

        fn sits(&self) -> u32 {
            println!("Victorian Sofa has {} sits", self.sits);
            self.sits
        }
    }

    impl Sofa for ModernSofa {
        fn has_back(&self) -> bool {
            println!("Modern Sofa has back");
            true
        }

        fn sits(&self) -> u32 {
            println!("Modern Sofa has {} sit(s)", self.sits);
            self.sits
        }
    }

    //
    pub struct VictorianFactory;

    pub struct ModernFactory;

    impl FurnitureFactory for VictorianFactory {
        type Chair = VictorianChair;
        type CoffeeTable = VictorianCoffeeTable;
        type Sofa = VictorianSofa;

        fn create_chair(&self) -> VictorianChair {
            VictorianChair {
                occupied: false,
                legs: 0,
            }
        }

        fn create_coffee_table(&self) -> VictorianCoffeeTable {
            VictorianCoffeeTable { coffees: 4 }
        }

        fn create_sofa(&self) -> VictorianSofa {
            VictorianSofa { sits: 5 }
        }
    }

    impl FurnitureFactory for ModernFactory {
        type Chair = ModernChair;
        type CoffeeTable = ModernCoffeeTable;
        type Sofa = ModernSofa;

        fn create_chair(&self) -> ModernChair {
            ModernChair { occupied: true }
        }

        fn create_coffee_table(&self) -> ModernCoffeeTable {
            ModernCoffeeTable { coffees: 5 }
        }

        fn create_sofa(&self) -> ModernSofa {
            ModernSofa { sits: 6 }
        }
    }
}

/// mod single
#[cfg(test)]
mod design_patterns_tests {
    use crate::learning::design_patterns::abstract_factory::{
        Chair, FurnitureFactory, ModernChair, ModernFactory,
    };

    #[test]
    fn abstract_factory_tests() {
        let modern_factory = ModernFactory;
        let chair: ModernChair = modern_factory.create_chair();
        assert_eq!(chair.has_legs(), true);
    }
}
