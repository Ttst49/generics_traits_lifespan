use std::fmt::Display;

fn main() {
    test_lifespan_and_make_error()
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
fn find_greater<T: PartialOrd + Copy>(list: &[T])->T{

    let mut greater_value = list[0];

    for &value in list {
        if value > greater_value {
            greater_value = value;
        }
    }
    greater_value

}

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

//start trait section

pub trait Summarizable{

    fn resume_author(&self)->String;
    fn summarize(&self)->String{
        format!("{}, en savoir plus ...", self.resume_author())
    }
}


pub struct PressArticle {
    pub title: String,
    pub place: String,
    pub author: String,
    pub content: String
}

// if no definition of trait inside impl, it use the default value
impl Summarizable for PressArticle{
    fn resume_author(&self) -> String {
        format!("L'auteur est {}", self.author)
    }
}



pub struct Tweet{
    pub username: String,
    pub content: String,
    pub response: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn resume_author(&self) -> String {
        format!("Auteur est {}", self.username)
    }

    fn summarize(&self) -> String {
        format!("tweet de {}, il dit: {}",self.username,self.content)
    }
}

//using trait inside a function
fn using_trait(){
    let tweet = Tweet{
        username: String::from("john"),
        content: String::from("Il y a actuellement 7 merveilles dans le monde"),
        response: true,
        retweet: true,
    };

    println!("1 nouvelle notification: {}",tweet.summarize());

    let press_article = PressArticle{
        title: String::from("partie gratuite au laser game de Nice"),
        place: String::from("Nice"),
        author: String::from("Philippe Henry"),
        content: String::from("Il y a actuellement une partie gratuite de laser game de Nice sur demande au journal"),
    };

    println!("1 nouvel article de presse: {}", press_article.summarize())
}

//can add implementation in a function, then it can only accept variable that got this trait
fn notifier(el : &impl Summarizable){
    println!("Eh oh regarde ça {}",el.summarize())
}

//can make it more clear and working for each params of a function
fn notifier_with_linked_trait<T: Summarizable + Display>(el : &T){
    println!("Flash info ! {}", el.summarize());
}

// you can either use this way
//     fn une_fonction<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// or this one
fn function<T,U>(t: &T, u: &U)->i64
where
    T: Display + Clone,
    U: Display + Summarizable
{
    25
}

//you can use as a return in a function the trait
fn return_summarizable()-> impl Summarizable{
    Tweet{
        username: String::from("louis"),
        content: String::from("coucou"),
        response: false,
        retweet: false,
    }
}

//using linked_trait with only specific method in impl

struct Pair<T>{
    x:T,
    y:T
}

impl<T> Pair<T>{
    fn new(x:T, y:T)->Self{
        Self{ x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T>  {
    fn compare(&self) {
        if self.x >= self.y {
            println!("Le plus grand élément est x = {}", self.x);
        } else {
            println!("Le plus grand élément est y = {}", self.y);
        }
    }
}


//lifespan start here


fn test_lifespan_and_make_error(){

    let r;

    {
        let x = 5;
        r = &x;
    }//x stop here, then r can't keep &x value and it panic

    println!("r: {}",r)
}

