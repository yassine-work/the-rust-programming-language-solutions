/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
    */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height

    }

    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>= other.width && self.height>=other.height


    }
    fn square(size:u32)->Self{
        Self{
            width:size,
            height:size,
        }
    }

}







fn main() {
    /*
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}",user1.active);
    ______________________________________________________*/
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("{rect1:#?}");
    //dbg!(&rect1);
    println!("{}",rect1.area());
    if rect1.width(){
        println!("test passed");
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    
    
       

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle{
    fn width(&self)->bool{
        self.width>0 
    }

}

