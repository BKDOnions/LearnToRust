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
        // In traits, `type` is used to declare an associated type:
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

    // Concrete Coffee Table variant 2
    pub struct ModernCoffeeTable {
        coffees: u32,
    }

    // Concrete Sofa variant 1
    pub struct VictorianSofa {
        sits: u32,
    }

    // Concrete Sofa variant 2
    pub struct ModernSofa {
        sits: u32,
    }

    // VictorianChair implementation
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

    // Optional VictorianChair implementation
    impl VictorianChair {
        fn legs(&self) -> u32 {
            println!("VictorianChair has {} legs", self.legs);
            self.legs
        }
    }

    // ModernChair implementation
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

    // Victorian Coffee Table implementation
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

    // Modern Coffee Table implementation
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

    // Victorian Sofa implementation
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

    // Modern Sofa implementation
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

    // Concrete Victorian Factory
    pub struct VictorianFactory;

    // Concrete Modern Factory
    pub struct ModernFactory;

    // Implementations
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

/// ## Builder
///
/// > Builder is a creational design pattern that lets you construct complex objects step by step.
/// The pattern allows you to produce different types and representations of an object using the same
/// construction code.
///
/// ### Problem
///
/// Imagine there is an object that has multiple fields with way too much parameters to construct,
/// or even worse: scattered all over the client code. Like the House below: way too many parts,
/// but some of them are unnecessary
///  
/// ![Multiple Fields Object](https://refactoring.guru/images/patterns/diagrams/builder/problem2.png)
///
/// ### Solution
/// The Builder pattern suggests that you **extract** the object construction code out of
/// its own class and **move** it to separate objects called **builders**.
///
/// Some of the construction steps might require different implementation when you need to
/// build various representations of the product. For example, walls of a cabin may be built of wood,
/// but the castle walls must be built with stone.
///
/// ![Builder with Director](https://refactoring.guru/images/patterns/diagrams/builder/structure.png)
///
///
mod builder {
    // HouseBuilder trait, it has similar steps to build a house.
    pub trait HouseBuilder {
        fn new() -> Self;
        fn build_windows(&mut self, windows: u32, component: &str) -> &mut Self;
        fn build_doors(&mut self, doors: u32, component: &str) -> &mut Self;
        fn build_walls(&mut self, walls: u32, component: &str) -> &mut Self;
        fn build(&mut self) -> &mut Self;
    }

    // Concrete house 1 Hotel Room.
    pub struct HotelRoom {
        pub(self) windows: (u32, String),
        pub(self) doors: (u32, String),
        pub(self) walls: (u32, String),
        pub(self) bathrooms: u32,
    }

    // Concrete house 2 Cabin
    pub struct Cabin {
        pub(self) windows: (u32, String),
        pub(self) doors: (u32, String),
        pub(self) walls: (u32, String),
        pub(self) garden: bool,
    }

    // Implement HouseBuilder for Hotel Room, aka Hotel Room Builder
    impl HouseBuilder for HotelRoom {
        fn new() -> Self {
            println!("new Hotel Room");
            Self {
                windows: (0, "".to_string()),
                doors: (0, "".to_string()),
                walls: (0, "".to_string()),
                bathrooms: 0,
            }
        }

        fn build_windows(&mut self, windows: u32, component: &str) -> &mut Self {
            println!("build windows");
            self.windows = (windows, String::from(component));
            self
        }

        fn build_doors(&mut self, doors: u32, component: &str) -> &mut Self {
            println!("build doors");
            self.doors = (doors, String::from(component));
            self
        }

        fn build_walls(&mut self, walls: u32, component: &str) -> &mut Self {
            println!("build walls");
            self.walls = (walls, String::from(component));
            self
        }

        fn build(&mut self) -> &mut Self {
            self
        }
    }

    impl HotelRoom {
        pub fn build_bathrooms(&mut self, bathrooms: u32) -> &mut Self {
            println!("build bathroom");
            self.bathrooms = bathrooms;
            self
        }
    }

    // Implement HouseBuilder for Cabin, aka Cabin Builder
    impl HouseBuilder for Cabin {
        fn new() -> Self {
            println!("new Cabin");
            Self {
                windows: (0, "".to_string()),
                doors: (0, "".to_string()),
                walls: (0, "".to_string()),
                garden: false,
            }
        }

        fn build_windows(&mut self, windows: u32, component: &str) -> &mut Self {
            println!("build windows");
            self.windows = (windows, String::from(component));
            self
        }

        fn build_doors(&mut self, doors: u32, component: &str) -> &mut Self {
            println!("build doors");
            self.windows = (doors, String::from(component));
            self
        }

        fn build_walls(&mut self, walls: u32, component: &str) -> &mut Self {
            println!("build walls");
            self.windows = (walls, String::from(component));
            self
        }

        fn build(&mut self) -> &mut Self {
            self
        }
    }

    impl Cabin {
        pub fn build_garden(&mut self, garden: bool) -> &mut Self {
            println!("build Cabin garden");
            self.garden = garden;
            self
        }
    }
}

#[cfg(test)]
mod design_patterns_tests {
    use crate::learning::design_patterns::abstract_factory::{
        Chair, CoffeeTable, FurnitureFactory, ModernChair, ModernFactory, ModernSofa, Sofa,
        VictorianCoffeeTable, VictorianFactory,
    };
    use crate::learning::design_patterns::builder::{Cabin, HotelRoom, HouseBuilder};

    #[test]
    fn abstract_factory_tests() {
        let modern_factory: ModernFactory = ModernFactory;
        let modern_chair: ModernChair = modern_factory.create_chair();
        let modern_sofa: ModernSofa = modern_factory.create_sofa();
        assert_eq!(modern_chair.has_legs(), true);
        assert_eq!(modern_sofa.sits(), 6);
        let victorian_factory: VictorianFactory = VictorianFactory;
        let victorian_coffee_table: VictorianCoffeeTable = victorian_factory.create_coffee_table();
        assert_eq!(victorian_coffee_table.has_coffees(), 4);
    }

    #[test]
    fn builder_tests() {
        let mut cabin: Cabin = Cabin::new();
        cabin
            .build_windows(4, "Glass")
            .build_walls(4, "Whites")
            .build_doors(4, "Wood")
            .build_garden(true);
        let mut hotel_room: HotelRoom = HotelRoom::new();
        hotel_room
            .build_walls(8, "Whites")
            .build_doors(3, "Steel")
            .build_windows(4, "Glass")
            .build_bathrooms(2);
    }
}
