macro_rules! with_str {
    ($name:ident, $func:ident) => {
        pub fn $func(&mut self, $name: &str) {
            self.bicycle.$name = $name.into()
        }
    };
}

macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        pub fn $func(&mut self, $name: $type) {
            self.bicycle.$name = $name
        }
    };
}

macro_rules! accessor {
    ($name:ident, &$ret:ty) => {
        // ret: return type
        pub fn $name(&self) -> &$ret {
            &self.$name
        }
    };
    ($name:ident,$ret:ty) => {
        pub fn $name(&self) -> $ret {
            self.$name
        }
    };
}

fn main() {
    // let mut bicycle_builder = BicycleBuilder::new();
    // bicycle_builder.with_make("中国连云港");
    // bicycle_builder.with_model("鳯凰");
    // bicycle_builder.with_size(42);
    // bicycle_builder.with_colour("天蓝色");

    // let bicycle = bicycle_builder.build();
    // println!("我的新自行车：{:?}", bicycle);

    let mut bicycle_builder = Bicycle::builder();
    bicycle_builder.with_make("中国上海");
    bicycle_builder.with_model("鳯凰牌");
    bicycle_builder.with_size(42);
    bicycle_builder.with_colour("天蓝色");

    let bicycle = bicycle_builder.build();
    println!("我的新自行车: {:?}", bicycle);
}

trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}

trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

#[derive(Debug, Default)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    colour: String,
}

impl Buildable<Bicycle, BicycleBuilder> for Bicycle {
    fn builder() -> BicycleBuilder {
        BicycleBuilder::new()
    }
}

impl Bicycle {
    // fn make(&self) -> &String {
    //     &self.make
    // }
    // fn model(&self) -> &String {
    //     &self.model
    // }
    // fn size(&self) -> i32 {
    //     self.size
    // }
    // fn colour(&self) -> &String {
    //     &self.colour
    // }

    // 使用宏生成结构体方法
    accessor!(make, &String);
    accessor!(model, &String);
    accessor!(size, i32);
    accessor!(colour, &String);
}

struct BicycleBuilder {
    bicycle: Bicycle,
}

impl BicycleBuilder {
    // fn with_make(&mut self, make: &str) {
    //     self.bicycle.make = String::from(make)
    // }
    // fn with_model(&mut self, model: &str) {
    //     self.bicycle.model = model.into()
    // }
    // fn with_size(&mut self, size: i32) {
    //     self.bicycle.size = size
    // }
    // fn with_colour(&mut self, colour: &str) {
    //     self.bicycle.colour = colour.into()
    // }

    // 使用宏生成结构体方法
    with_str!(make, with_make);
    with_str!(model, with_model);
    with!(size, with_size, i32);
    with_str!(colour, with_colour);
}

impl Builder<Bicycle> for BicycleBuilder {
    fn new() -> Self {
        Self {
            bicycle: Bicycle::default(),
        }
    }
    fn build(self) -> Bicycle {
        self.bicycle
    }
}
