use std::io;

fn pounds_to_kilogram(pounds_value: f32) -> f32{
    pounds_value * 2.20462
}

fn inches_to_metres(inches_value: f32) -> f32{
    inches_value / 39.3701
}

fn foot_to_metres(foot_value: f32) -> f32{
    foot_value / 3.28084
}

fn centimeter_to_metre(centimeter_value: f32) -> f32{
    centimeter_value / 100.0
}
#[derive(Debug)]
struct Measurements{
    weight: f32,
    height: f32,
}

impl Measurements {
    fn converter(&self) -> f32{
        let height_squared = self.height * self.height;
        self.weight / height_squared
    }
}


fn main() {
    loop{
        println!("BMI calculator");
        println!("Input your weight(KG), and your height(M)");
        println!("Enter 'M' if you want to use any of our measurement unit converter or click on Enter to continue");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice:char = choice.trim().parse().expect("Invalid input");
        if choice == 'M'{

        }

        else if choice == ' '{
            println!("Weight(KG): ");
            let mut user_weight = String::new();
            io::stdin()
                .read_line(&mut user_weight)
                .expect("Failed to read line");
            let user_weight:f32 = user_weight.trim().parse().expect("Invalid input, please enter a positive integer");
        
            println!("Height(M): ");
            let mut user_height = String::new();
            io::stdin()
                .read_line(&mut user_height)
                .expect("Failed to read line");
            let user_height:f32 = user_height.trim().parse().expect("Invalid input, please enter a positive integer");
        
            let user_measurement = Measurements{
                weight: user_weight,
                height: user_height
            };
        
            println!("Your BMI index based on your weight of {}, and height of {} is {:.2}", user_weight, user_height, user_measurement.converter())
        
        }      
        
    }

}
