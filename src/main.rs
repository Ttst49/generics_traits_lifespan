fn main() {
    launch_greater_application()
}


fn launch_greater_application(){
    let number_list = vec![13,273,283,109,588,637];

    println!("Le plus grand nombre est {}",find_greater(&number_list));

    let characters_list = vec!['y','x','v','h','d','e'];

    println!("Le plus grand caractère est {}",find_greater(&characters_list));
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
fn find_greater<T>(list: &[T])->T{

    let mut greater_value = list[0];

    for &value in list {
        if value > greater_value {
            greater_value = value;
        }
    }
    greater_value

}