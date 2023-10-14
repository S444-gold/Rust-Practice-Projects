use std::io;

fn pounds_to_kilogram(pounds_value: f32) -> f32 {
    pounds_value / 2.20462
}

fn inches_to_metres(inches_value: f32) -> f32 {
    inches_value / 39.3701
}

fn foot_to_metres(foot_value: f32) -> f32 {
    foot_value / 3.28084
}

fn centimeter_to_metre(centimeter_value: f32) -> f32 {
    centimeter_value / 100.0
}

fn get_float_input(prompt: &str) -> f32{
    loop{
        println!("{}", prompt);
        let mut float_input = String::new();
        io::stdin()
            .read_line(&mut float_input)
            .expect("Failed to read line");

        match float_input.trim().parse::<f32>(){
            Ok(value) if value > 0.0 => return value,
            _ => println!("Please enter a positive integer")
        };
    }
}//simple function that can be passed around to take user 'float' input, to make code shorter and readable

fn get_char_input(prompt: &str) -> char{
    loop{
        println!("{}", prompt);
        let mut char_input = String::new();
        io::stdin()
            .read_line(&mut char_input)
            .expect("Failed to read line");

        match char_input.trim().parse::<char>(){
            Ok(input) => return input,
            _ => println!("Wrong input, try again")
        }
    }
}//simple function that can be passed around to take user 'char' input, to make code shorter and readable 

#[derive(Debug)]
struct Measurements {
    weight: f32,
    height: f32,
}

impl Measurements {
    fn converter(&self) -> f32 {
        let height_squared = self.height * self.height;
        self.weight / height_squared
    }
}

fn main() {
    loop {
        println!("BMI calculator");
        println!("A measurement unit converter as being added just incase you don't know your weight or height measurement in our preferred Units");
        let choice = get_char_input("Enter 'M' to use measurement unit converter\nEnter 'B' to continue with the BMI calculation");
        
        if choice == 'M' {
            let  converter_choice = get_char_input("Enter (W) for weight unit converter or (H) for height unit converter");

            if converter_choice == 'W' {
                let lbs_weight = get_float_input("Input your weight in pounds(lbs) to be converted to Kilograms(KG): ");
                {println!("{}(lbs) is {:.1}KG", lbs_weight, pounds_to_kilogram(lbs_weight));
                    break;}
                    
            } else if converter_choice == 'H' {
                //Conversion of some length measurement unit to metre
                let height_choice = get_char_input("Enter:\n(I)to convert from inches to meters\n(F)to convert from foot to metres\n(C)to convert from centimetre to metre");
                
                if height_choice == 'I' {
                    let height_inches = get_float_input("Input your height in inches to be converted to meters: ");
                    {println!("{}\" is {:.2}m", height_inches, inches_to_metres(height_inches));
                        break;}

                } else if height_choice == 'F' {
                    let height_foot = get_float_input("Input your height in feet to be converted to meters: ");
                    {println!("{}ft' is {:.2}m", height_foot, foot_to_metres(height_foot));
                        break}
                } else if height_choice == 'C' {
                    let height_cm = get_float_input("Input your height in centimeters to be converted to meters: ");
                    {println!("{}cm is {:.2}m", height_cm, centimeter_to_metre(height_cm));
                        break;}
                }
            }
            //Continuation of the BMI code, incase user wants to jump straight into it
        } else if choice == 'C' {
            println!("Kindly enter your weight in Kilograms and height in metres");
            let user_weight = get_float_input("Weight(KG)");
            let user_height = get_float_input("Height(M)");
            let user_measurement = Measurements {
                weight: user_weight,
                height: user_height,
            };
            
            {println!("Your BMI index based on your weight of {}kg, and height of {}m is {:.2}", user_weight, user_height, user_measurement.converter(),);
                
                //For output based on the users BMI score
                if user_measurement.converter() < 18.5{
                    println!("Underweight!!!!!")
                }

                else if user_measurement.converter() < 24.9{
                    println!("Healthy Weight!")
                }
                
                else{
                    println!("OVERWEIGHT!!!!!")
                }
                break;}
        }
    }
}