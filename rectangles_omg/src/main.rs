#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    area: u32,
}

fn create_rectangle(width: u32, height:u32) -> Rectangle {
    Rectangle {width, height, area: width*height}
}

// if we dont want the area to be written in the struct when it is created we can calculate it
// dinamically (idk how to write XD)

struct _Rectangle {
    width: u32,
    height: u32,
}

impl _Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &_Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width: u32 = 10;
    let height: u32 = 3;
    let rectangle = create_rectangle(width, height);
    
    println!("Thea rea of the rectangle is: {}", rectangle.area);    
    // You can print the whole struct with 
    //     println!({:?}, rectangle);
    // but you can add #[derive(Debug)] in the def of the struct and use
    dbg!(rectangle);
    // imo this is better


    // New struct with the impl of rectangle
    let rect = _Rectangle{width, height};
    println!("Thea rea of the rectangle is: {}", rect.area());    


}
