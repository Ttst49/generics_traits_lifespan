fn main() {
    launch_greater_application()
}


fn launch_greater_application(){
    let number_list = vec![13,273,283,109,588,637];

    println!("Le plus grand nombre est {}",find_greater_number_i64(&number_list));

    let characters_list = vec!['y','x','v','h','d','e'];

    println!("Le plus grand caractère est {}",find_greater_char(&characters_list));
}

//function that return i64 value
fn find_greater_number_i64(list: &[i64])->i64{
    let mut greater_number = list[0];

    for &number in list {
        if number > greater_number {
            greater_number = number
        }
    }
    greater_number
}

//function that return char value
fn find_greater_char(list : &[char])->char{
    let mut greater_char = list[0];

    for &char in list {
        if char > greater_char{
            greater_char = char
        }
    }
    greater_char
}

//generic function that return the greater value from a T generic variable
//fn find_greater<T>(list: &[T])->T{

  //  let mut greater_value = list[0];

    //for &value in list {
      //  if value > greater_value {
        //    greater_value = value;
        //}
    //}
    //greater_value

//}

//start generic in struct
struct Point<T>{
    x:T,
    y:T
}
//can't use 2 different types with the same generic variable

struct PointWith2Variables<A,B>{
    x:A,
    y:B
}

//using generic in impl
impl<T> Point<T> {
    fn x(&self)->&T{
        &self.x
    }
    fn y(&self)->&T{
        &self.y
    }
}

fn create_point_from_generic_struct(){
    let ints = Point{ x:12, y:89 };
    let floats = Point{ x:73.34, y:954.5 };

    //example 2 different type with same generic variable
    //let error = Point{ x: 12, y: 26.2 };
    //example 2 different type with different generic variable
    //let good = PointWith2Variables{ x:12, y:16.5 };

    //using generic function from impl
    println!("int.x ={} et int.y = {}",ints.x(),floats.y())

}
