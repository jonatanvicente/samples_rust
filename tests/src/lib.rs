pub fn add_two(a: i32) -> i32 {
    a + 3
}


#[cfg(test)]
mod tests {
    use super::*; //necestiamos importar el modulo para poder usarlo

#[test]
fn exploration() {
    assert_eq!(4, add_two(2), "Mensaje de error personalizado"); //da igual el orden de la aserción
}
    
    /*    #[test]
		fn another() {
			panic!("Make this test fail");  //hacemos fallar intencional
		}*/
    
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(!smaller.can_hold(&larger));
    }
}


