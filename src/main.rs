fn main() {
    launch_greater_application()
}


fn launch_greater_application(){
    let number_list = vec![13,273,283,109,588,637];

    println!("Le plus grand nombre est {}",find_greater_number(&number_list))
}

fn find_greater_number(list: &[i64])->i64{
    let mut greater_number = list[0];

    for &number in list {
        if number > greater_number {
            greater_number = number
        }
    }
    greater_number
}