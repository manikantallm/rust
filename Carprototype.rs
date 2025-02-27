// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    age: (Age,u32),
}


#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}
#[derive(Debug)]
enum Age{
    New,
    Old,
}

fn car_factory(_color:String,_trans:Transmission,_conver:bool,_miles:u32)->Car{
    let car:Car = Car{
        color:_color,
        transmission:_trans,
        convertible:_conver,
        //mileage:0,
        age:car_quality(_miles),
    };
    car
}



fn car_quality(miles:u32)->(Age,u32){

    if miles==0{
        return (Age::New,miles);
    }
    (Age::Old,miles)
}

fn main(){
    println!("The maincode is executed");
    let colors = ["Red","Silver","Blue","Green"];
    
    let mut car:Car = car_factory(String::from(colors[0]), Transmission::Manual, false,0);
   println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {:?}", car.color, car.transmission, car.convertible, car.age);

    car = car_factory(String::from(colors[1]), Transmission::Automatic, true,100);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {:?}", car.color, car.transmission, car.convertible, car.age);

    car = car_factory(String::from(colors[2]), Transmission::SemiAuto, false,200);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {:?}", car.color, car.transmission, car.convertible, car.age);    

}