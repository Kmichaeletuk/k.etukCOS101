// Rust program to develop an application that will
// crate an active database for members of globacom
use std::io;
use std::io::Read;

fn open_globacom_dbase()
{
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_project_tb()
{
    let mut file = std::fs::File::open("project_tb.sql"
    	).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_staff_tb()
{
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_customer_tb()
{
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_data_plan_tb()
{
    let mut file = std::fs::File::open("data-plan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main()
{
    println!("Which is your position below:\n");
    println!("Input 1 for Administrator");
    println!("Input 2 for Project Manager");
    println!("Input 3 for Employee");
    println!("Input 4 for Customer");
    println!("Input 5 for Data-Plan\n");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let position: i32 = choice.trim().parse().expect("Failed to input");

    if position == 2
    {
        println!("This is the Database structure");
        open_globacom_dbase();
    }

    if position == 1
    {
        println!("This is the Project table");
        open_project_tb();
    }

    if position == 4
    {
        println!("This is the Staff table");
        open_staff_tb();
    }

    if position == 3
    {
        println!("This is the Customer table");
        open_customer_tb();
    }

    if position == 5
    {
        println!("This is the Data-Plan table");
        open_data_plan_tb();
    }

}
