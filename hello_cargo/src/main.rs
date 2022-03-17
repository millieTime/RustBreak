use std::io;            // Allows user input
use rand::Rng;          // Random Numbers
use std::cmp::Ordering; // Match

// The following line is a decorator.
#[derive(Debug)]  // Allows {:?} to display contents
// Like a .h file in c++
struct Employee {
  name : String,
  pay : f32
}
// Like a cpp file in c++
impl Employee {
  fn disp(& self) {
    println!("{:?}", self);
    println!("My name is {} and my pay is {}", self.name, self.pay);
  }
  fn change_pay(&mut self, amount: f32){
    self.pay += amount;
  }
}



// Ampersand means "You can borrow it for your function, but give it back".
fn disp_employee(employee: & Employee){ 
    println!("{:?}", employee);
}

fn change_pay(employee: &mut Employee, amount: f32) {  
    employee.pay += amount;
}


// must specify inputs and output
fn add(a : i32, b : i32)  -> i32{
  return a + b;
}
  
fn main() {

    println!("Hello World!");
    let x :i32 = 10;
    let mut y = 20;
    println!("x={} y={}", x, y);    
    //x = 20; // Illegal because x is immutable
    y = 40;
    println!("x={} y={}", x, y);

    if x > y {
      println!("Lesser");
    } else if x > y {
      println!("Greater");
    } else {
      println!("Equal");
    }
  
    println!("x + y = {}", add(x, y));

    
    let mut e = Employee {
        name:String::from("Bob"),
        pay:49.95
    };
            
    disp_employee(& e);
    change_pay(&mut e, 20.00); 
    disp_employee(& e);
    println!("==============");
    

    let mut team: Vec<Employee> = Vec::new();
    team.push(Employee { name:String::from("Tim"), pay:60.0 });
    team.push(Employee { name:String::from("Sue"), pay:70.0 });
    team.push(Employee { name:String::from("Ben"), pay:80.0 });

    // By default, rust iters give &vars (so they transfer scope)
    // iter_mut gives mut &vars
    for emp in team.iter_mut() {
      emp.disp();
      emp.change_pay(100.0);
      emp.disp();
    }


    // PART 2

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101); // the upper bound of range is exclusive unless (1..=100)
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}