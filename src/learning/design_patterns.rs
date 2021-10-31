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

/// ## Factory Method
///
/// > Factory Method is a creational design pattern that provides an interface for creating objects
/// in a superclass, but allows subclasses to alter the type of objects that will be created.
///
/// ### Problem
/// Imagine you have a company for transportation, and you want to start your business with
/// road transporting, then as the business growing, you receive multiple other form of
/// transportations orders, while your company only knows the road way mostly, and you don't
/// have the money and mind to build another companies.
///
/// ### Solution
/// The Factory Method pattern suggests that you replace direct object construction calls
/// (using the new operator) with calls to a special factory method. Don’t worry: the objects
/// are still created via the new operator, but it’s being called from within the factory method.
/// Objects returned by a factory method are often referred to as products.
/// ![Subclasses can alter the class of objects being returned by the factory method.](https://refactoring.guru/images/patterns/diagrams/factory-method/solution1.png)
///
///
///

mod factory_method {
    pub trait Transport {
        fn deliver(&self);
    }

    pub struct RoadTrunk {
        payload: String,
        boxes: u32,
    }

    pub struct SeaShip {
        payload: String,
        containers: u32,
    }

    impl Transport for RoadTrunk {
        fn deliver(&self) {
            println!("Delivering Payload with road transportations in boxes");
        }
    }

    impl Transport for SeaShip {
        fn deliver(&self) {
            println!("Delivering payload with sea transportations in containers");
        }
    }

    impl RoadTrunk {
        pub fn new(payload: &str, boxes: u32) -> Self {
            RoadTrunk {
                payload: String::from(payload),
                boxes,
            }
        }
    }

    impl SeaShip {
        pub fn new(payload: &str, containers: u32) -> Self {
            SeaShip {
                payload: String::from(payload),
                containers,
            }
        }
    }
}

/// ## Singleton
///
/// ### Intent
///
/// **Singleton** is a creational design pattern that lets you ensure that a class has
/// only one instance, while providing a global access point to this instance.
///
/// ### Note
/// > The Singleton pattern solves two problems at the same time,
/// violating the Single Responsibility Principle
///
/// ### Solution
/// + Ensure that a class has just a single instance.
/// + Provide a global access point to that instance.
/// + Make the default constructor private,
/// to prevent other objects from using the new operator with the Singleton class.
/// + Create a static creation method that acts as a constructor. Under the hood,
/// this method calls the private constructor to create an object and saves it in a static field.
/// All following calls to this method return the cached object.
///
///
///

mod singleton {
    /// Useless in rust
}

/// # Structural Patterns
///
/// ## Adapter (Wrapper)
///
/// ### Intent
///
/// **Adapter** is a design pattern that allows object with incompatible interfaces to collaborate
///
/// ### Problem
/// Imagine that you’re creating a stock market monitoring app. You have solve most of
/// the app logic problems, then you start coding but find out all the markets are using
/// xml format to pass data to the FE.
///
/// ### Solution
/// You can create an adapter. This is a special object that converts the interface of one object
/// so that another object can understand it.
///
/// Adapters can not only convert data into various formats but can also help objects with
/// different interfaces collaborate. Here’s how it works:
///
/// + The adapter gets an interface, compatible with one of the existing objects.
/// + Using this interface, the existing object can safely call the adapter’s methods.
/// + Upon receiving a call, the adapter passes the request to the second object,
/// but in a format and order that the second object expects.
///
/// ![Object adapter](https://refactoring.guru/images/patterns/diagrams/adapter/structure-object-adapter.png)
///

mod adapter {
    // Round Hole
    pub struct RoundHole {
        radius: f32,
    }

    // Round Peg
    pub struct RoundPeg {
        radius: f32,
    }

    // Square Peg
    pub struct SquarePeg {
        width: f32,
    }

    // Adapter
    pub struct SquarePegAdapter {
        peg: SquarePeg,
    }

    // Round Hole Implementations: as it only fits which radius is less than its own radius
    impl RoundHole {
        pub fn new(radius: f32) -> Self {
            RoundHole { radius }
        }
        pub fn get_radius(&self) -> f32 {
            self.radius
        }
        pub fn fit(&self, peg: RoundPeg) -> bool {
            peg.radius <= self.radius
        }
    }

    //
    impl RoundPeg {
        pub fn new(radius: f32) -> Self {
            RoundPeg { radius }
        }
        pub fn get_radius(&self) -> f32 {
            self.radius
        }
    }

    impl SquarePeg {
        pub fn new(width: f32) -> Self {
            SquarePeg { width }
        }
        pub fn get_width(&self) -> f32 {
            self.width
        }
    }

    // Get square peg radius
    impl SquarePegAdapter {
        pub fn new(peg: SquarePeg) -> Self {
            SquarePegAdapter { peg }
        }
        pub fn get_radius(&self) -> f32 {
            self.peg.get_width() * 2_f32.sqrt() / 2_f32
        }
    }
}

/// # Behavioral Patterns
///
/// ## Observer (Event Subscriber/Listener)
///
/// ### Intent
///
/// > **Observer** is a behavioral design pattern that lets you define a subscription mechanism
/// to notify multiple objects about any events that happen to the object they’re observing.
///
/// ### Problem
///
/// Imagine you have a Store. In past, people will come to your Store to check product availability,
/// if the product is still on their way, the trip will be pointless, nowadays, as technology
/// developed, we can keep our customer well informed by sending emails, but the we can never know
/// each customer's demand unless the customer tell us.
///
/// ### Solution
/// Create a subscription list for each product, the manager notify the subscribers in the list
/// when the product is available.
///
/// ### Structure
/// ![Observer](https://refactoring.guru/images/patterns/diagrams/observer/structure-indexed.png)
///

mod observer {
    use std::collections::{HashMap, HashSet};
    use std::hash::{Hash, Hasher};

    // Subscriber
    pub trait EventListener {
        fn update(&self, event_type: String, filename: String);
    }

    // Publisher
    pub struct EventManager<T: EventListener> {
        listeners: HashMap<String, HashSet<T>>,
    }

    // Concrete Publisher
    pub struct Editor<T: EventListener> {
        pub event_manager: EventManager<T>,
        filename: String,
    }

    // Concrete Listener
    pub struct EmailNotificationListener {
        email: String,
    }

    // Implement Publisher with Hash, Eq traits for HashSet
    impl<T: EventListener + Hash + Eq> EventManager<T> {
        pub fn new(operations: Vec<String>) -> EventManager<T> {
            let mut event_manager = EventManager {
                listeners: HashMap::new(),
            };
            for operation in operations.iter() {
                event_manager
                    .listeners
                    .insert(String::from(operation), HashSet::new());
            }
            event_manager
        }
        pub fn subscribe(&mut self, event_type: &str, listener: T) {
            self.listeners
                .get_mut(event_type)
                .expect("Event Type Not Found")
                .insert(listener);
        }
        pub fn unsubscribe(&mut self, event_type: &str, listener: T) {
            self.listeners
                .get_mut(event_type)
                .expect("Event Type Not Found")
                .remove(&listener);
        }
        pub fn notify(&self, event_type: &str, filename: String) {
            let users = self
                .listeners
                .get(event_type)
                .expect("Event Type Not Found");
            for user in users {
                user.update(String::from(event_type), String::from(&filename));
            }
        }
    }

    // Implement Subscriber for concrete subscriber
    impl EventListener for EmailNotificationListener {
        fn update(&self, event_type: String, filename: String) {
            println!(
                "Email To {}: Someone has performed {} on {} ",
                self.email, event_type, filename
            );
        }
    }

    // Implement concrete subscriber
    impl EmailNotificationListener {
        pub fn new(email: String) -> EmailNotificationListener {
            EmailNotificationListener { email }
        }
    }

    // Implement the supertrait of Eq for concrete subscriber
    impl PartialEq<Self> for EmailNotificationListener {
        fn eq(&self, other: &Self) -> bool {
            self.email.eq(&other.email)
        }
    }

    impl Eq for EmailNotificationListener {}

    // Implement Hash for concrete subscriber
    impl Hash for EmailNotificationListener {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.email.hash(state)
        }
    }

    // Implement concrete publisher
    impl<T: EventListener + Eq + Hash> Editor<T> {
        pub fn new() -> Editor<T> {
            Editor {
                event_manager: EventManager::new(vec![String::from("open"), String::from("close")]),
                filename: "".to_string(),
            }
        }
        pub fn open_file(&mut self, filename: String) {
            self.filename = String::from(&filename);
            self.event_manager.notify("open", String::from(&filename));
        }
        pub fn close_file(&self) {
            self.event_manager
                .notify("close", String::from(&self.filename));
        }
    }
}

/// ## Strategy Pattern
///
/// ### Intent
///
/// > **Strategy** is a behavioral design pattern that lets you define a family of algorithms,
/// put each of them into a separate class, and make their objects interchangeable.
///
/// ### Problem
///
/// Imagine there is an app, it requires register and login.
/// At first you have implemented phone number authentication. As the app grow, new users find it
/// hard to register than other app: they all has multiple authentication methods: phone number,
/// third party social account like google, facebook, twitter or any else that has concrete
/// authentication method.
///
/// ### Solution
///
/// The Strategy pattern suggests that you take a class that does something specific in a lot of
/// different ways and extract all of these algorithms into separate classes called strategies.
///
/// The original class, called context, must have a field for storing a reference to one of
/// the strategies. The context delegates the work to a linked Strategy object instead of
/// executing it on its own.
///
/// ![Structure](https://refactoring.guru/images/patterns/diagrams/strategy/structure.png)

mod strategy {
    pub trait Strategy {
        fn execute(&self, a: i32, b: i32) -> i32;
    }

    pub struct ConcreteAdd;

    pub struct ConcreteSubtract;

    pub struct ConcreteMultiply;

    pub struct Context {
        strategy: Box<dyn Strategy>,
    }

    impl Strategy for ConcreteAdd {
        fn execute(&self, a: i32, b: i32) -> i32 {
            a + b
        }
    }

    impl Strategy for ConcreteSubtract {
        fn execute(&self, a: i32, b: i32) -> i32 {
            a - b
        }
    }

    impl Strategy for ConcreteMultiply {
        fn execute(&self, a: i32, b: i32) -> i32 {
            a * b
        }
    }

    impl Context {
        pub fn new(strategy: Box<dyn Strategy>) -> Self {
            Context { strategy }
        }
        pub fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
            self.strategy = strategy;
        }
        pub fn execute_strategy(&self, a: i32, b: i32) -> i32 {
            self.strategy.execute(a, b)
        }
    }
}

#[cfg(test)]
mod design_patterns_tests {
    use crate::learning::design_patterns::abstract_factory::{
        Chair, CoffeeTable, FurnitureFactory, ModernChair, ModernFactory, ModernSofa, Sofa,
        VictorianCoffeeTable, VictorianFactory,
    };
    use crate::learning::design_patterns::adapter::{
        RoundHole, RoundPeg, SquarePeg, SquarePegAdapter,
    };
    use crate::learning::design_patterns::builder::{Cabin, HotelRoom, HouseBuilder};
    use crate::learning::design_patterns::factory_method::{RoadTrunk, SeaShip, Transport};
    use crate::learning::design_patterns::observer::{Editor, EmailNotificationListener};
    use crate::learning::design_patterns::strategy::{
        ConcreteAdd, ConcreteMultiply, ConcreteSubtract, Context,
    };

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

    #[test]
    fn factory_method_tests() {
        let trunk: RoadTrunk = RoadTrunk::new("Glass", 32);
        let ship: SeaShip = SeaShip::new("Gas", 42);

        trunk.deliver();
        ship.deliver();
    }

    #[test]
    fn adapter_tests() {
        let square_peg = SquarePeg::new(4_f32);
        let square_adapter = SquarePegAdapter::new(square_peg);
        let round_peg = RoundPeg::new(square_adapter.get_radius());
        let round_hole = RoundHole::new(4_f32);

        assert_eq!(round_hole.fit(round_peg), true);
    }

    #[test]
    fn observer_tests() {
        let mut editor = Editor::new();
        editor.event_manager.subscribe(
            "open",
            EmailNotificationListener::new("zqdbyct@gmail.com".to_string()),
        );
        editor.event_manager.subscribe(
            "close",
            EmailNotificationListener::new("zqdbyct@gmail.com".to_string()),
        );
        editor.open_file("test.txt".to_string());
        editor.close_file();
    }

    #[test]
    fn strategy_tests() {
        let a = 10;
        let b = 5;
        // Changing context
        let mut context = Context::new(Box::new(ConcreteAdd));
        assert_eq!(context.execute_strategy(a, b), 15);
        context.set_strategy(Box::new(ConcreteMultiply));
        assert_eq!(context.execute_strategy(a, b), 50);
        context.set_strategy(Box::new(ConcreteSubtract));
        assert_eq!(context.execute_strategy(a, b), 5);
    }
}
