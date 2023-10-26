
mod shapes{
    use std::f32::consts::PI;

    pub trait GeometricShape {
        fn area(&self) -> f32;
        fn to_string(&self) -> String;
    }
    
    pub trait RectangleShape{
        fn set_width(&mut self, width : f32);
        fn set_height(&mut self, height : f32);
    }
    
    pub struct Rectangle{
        width : f32,
        height : f32,
    }

    impl Rectangle{
        pub fn new(width : f32, height : f32) -> Self{
            Rectangle { width, height }
        }
    }

    impl GeometricShape for Rectangle{
        fn area(&self) -> f32 {
            self.width * self.height
        }
        fn to_string(&self) -> String {
            format!("Прямоугольнико с шириной и высотой: {}, {} и площадью: {}", self.width, self.height, self.area())
        }
    }

    impl RectangleShape for Rectangle {
        fn set_height(&mut self, height : f32) {
            self.height = height;
        }
        fn set_width(&mut self, width : f32) {
            self.width = width;
        }
    }
    
    pub struct Square{
        length : f32,
    }


    impl Square{
        #[warn(dead_code)]
        pub fn set_length(&mut self, length : f32){
            self.length = length;
        }

        pub fn new(length : f32) -> Self{
            Square { length }
        }
    }

    impl GeometricShape for Square{
        fn area(&self) -> f32 {
            self.length * self.length
        }
        fn to_string(&self) -> String {
            format!("Квадрат с стороной:{} и площадью: {}", self.length, self.area())
        }
    }

    impl RectangleShape for Square{
        fn set_height(&mut self, width : f32) {
            self.length = width;
        }
        fn set_width(&mut self, width : f32) {
            self.length = width;
        }
    }
    
    pub struct Circle{
        radius : f32,
    }
    impl Circle{
        pub fn new(radius : f32) -> Self{
            Circle { radius }
        }
    }

    impl GeometricShape for Circle{
        fn area(&self) -> f32 {
            self.radius * self.radius * PI 
        }
        fn to_string(&self) -> String {
            format!("Круг с радиусом:{} и площадью: {:.3}", self.radius, self.area())
        }
    }

}

macro_rules! print_shapes {
    ($shapes : expr, vector) => {
        for shape in $shapes{
            println!("{}", shape.to_string());
        }
        
    };
    
    ($($shape : expr), *) => {
        $(
            println!("{}", $shape.to_string());
        )*
        
    };

}

use std::f32::consts::PI;

use shapes::GeometricShape;

use crate::shapes::RectangleShape;


#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::shapes::Square;

    // Обратите внимание на эту полезную идиому: импортирование имён из внешней (для mod - тестов) области видимости.
    use super::*;

    #[test]
    fn test_rectangle() {
        let rec = shapes::Rectangle::new(10.0, 10.0);
        assert_eq!(rec.area(), 100.0);
    }

    #[test]
    fn test_print() {
        let square= shapes::Square::new(31.0);

        assert_eq!(square.to_string(), "Квадрат с стороной:31 и площадью: 961");
    }

    #[test]
    fn test_circle_bad() {
        let circle= shapes::Circle::new(10.0);
        assert_eq!(circle.area(), 314.1592);
    }
}


fn main() {

    let mut list_of_shapes:Vec<&mut dyn GeometricShape> = Vec::new();

    let mut square = shapes::Square::new(10.0);
    println!("{}", square.area());
    square.set_length(25.0);
    println!("{}", square.area());

    let mut rectangle = shapes::Rectangle::new(10.0, 20.0);
    println!("{}", rectangle.area());
    rectangle.set_width(25.0);
    println!("{}", rectangle.area());

    list_of_shapes.push(&mut square);
    list_of_shapes.push(&mut rectangle);

    print_shapes!(list_of_shapes, vector);  
    print_shapes!(square, rectangle);
}
