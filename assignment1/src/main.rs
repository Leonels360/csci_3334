//Leonel Saenz
//Problem 1
const FREEZE: f64 = 32.0;

fn fahernheit_to_celsius(f: f64) -> f64{
 (f - FREEZE) * (5.0 / 9.0)

}
fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * (9.0 / 5.0)) + FREEZE
}

fn tempereature_converter(){
    let mut current_fahrenheit: f64 = 68.0;
    println!("Starting temperature: ), current_fahrenheit");
    
    let celcius = fahernheit_to_celsius(current_fahrenheit);
    println!(
        "{:.1}°F converts to {:.2}°C",
        current_fahrenheit, celcius
    );


    println!("\nConverting the next 5 integer Fahrenheit temperature:");
    let mut counter = 0;

    loop {
        if counter >= 5{
            break;
        }
        
        current_fahrenheit += 1.0;

        let converted_celsius = fahernheit_to_celsius(current_fahrenheit);
        println!(
            "{}°F -> {:.2}°C",
            current_fahrenheit as i32, converted_celsius
        );
        
        counter += 1; 
    }
}

// problem 2
fn is_even(n: i32) -> bool {

    n % 2 == 0
}

fn number_analyzer(){
    let numbers: [i32; 10] = [1,2,3,4,5,6,7,8,15,30];
    
    println!("\nEven/Odd and Fizz/Buzz results:");
    for number in numbers {
        
        if number % 3 == 0 && number % 5 == 0{
            println!("{} is FizzBuzz", number);
        }

        else if number % 3 == 0 {
            println!("{} is Fizz", number);

        }
        else if number % 5 == 0 {
            println!("{} is Buzz", number);
        }
        else{
            if is_even(number){
            println!("{} is Even", number);
            }else{
            println!("{} is odd", number);
            }

        }
        
    }

    //calculate sum with loop
    println!("\nSum Calculation:");

    let mut sum: i32 = 0;
    let mut index: usize = 0;

    while index < numbers.len() {
        sum += numbers[index];

        index += 1;

    }
    println!("The sum of all numbers is: {}", sum);

    println!("\nLargest Number Search:");

    let mut largest: i32 = numbers[0];

    for number in numbers {

        if number > largest {
            largest = number;
        }
    }
    println!("The largest number in the array is: {}", largest);


}


// Problem number 3(the guessing game)

fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret {
        0
    }
    else if guess > secret {
        1
    } 
    else{
        -1
    }
}

fn guessing_game(){
    
    let secret_number: i32 = 42;
    println!("Start guessing!");

    let mut guesses_made: i32 = 0;
    let mut current_guess: i32;

    let simulated_guesses: [i32; 5] = [20, 50, 40, 45, 42];
    let mut guess_index: usize = 0;

    loop {
        guesses_made += 1; 

        current_guess = simulated_guesses[guess_index];
        guess_index += 1;

        println!("\nGuess #{}: {}", guesses_made, current_guess);

        let result = check_guess(current_guess, secret_number);


        if result == 0 {
            println!("You guessed correct, the secret_number was {}", secret_number);
            break;

        } else if result == 1 {
            println!("Your guess was too high. Try again.")
        } else {
            println!("Your guess was too low. Try again.");
        }
    }
    println!("\nIt took you {} guesses to find the secret numbers.", guesses_made);
}

fn main() {

    tempereature_converter();
    number_analyzer();
    guessing_game();

}