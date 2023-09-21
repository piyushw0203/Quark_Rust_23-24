use std::io;

fn main() 
{
    let mut flag=0;
    let mut n:i32;
    while flag!=1
    {
        println!("choose option\n1.Bubble Sort\n2.Selection Sort\n3.Linear search\n4.Exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        n=input[..].trim().parse::<i32>().unwrap();
    
        match n
        {
          1=>bubble_sort(),
          2=>selection_sort(),
          3=>linear_search(),
          4=>flag=1,
          _ =>println!("Enter correct input"),
        }
    }
}

fn bubble_sort() 
{
    let mut a: [i32;10]=[0;10];
    //accept arrray
    println!("enter the elements of array");
    for i in 0..9
    {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        a[i]=input[..].trim().parse::<i32>().unwrap();
    }
    for i in 0..9
    {
        for j in 0..8
        {
            if a[j]>a[j+1]
            {
                let mut temp=a[j];
                a[j]=a[j+1];
                a[j+1]=temp;
            }
        }
    }
    for i in 0..9
    {
        println!("{}",a[i]);
    }
}

fn selection_sort()
{
    let mut a:[i32;10]=[0;10];
    //    accept array
    println!("enter the elements of array");
    for i in 0..9
    {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        a[i]=input[..].trim().parse::<i32>().unwrap();
    }
    for i in 0..9
    {
        let mut index:usize=i;
        for j in i+1..9
        {
            if a[i]>a[j]
            {
                index=j;
            }
        }
        let temp=a[index];
        a[index]=a[i];
        a[i]=temp;
    }
    for i in 0..9
    {
        println!("{}",a[i]);
    }
}

fn linear_search()
{
    let mut a:[i32;10]=[0;10];
    let n:i32;
    let mut flag:i32=0;
    println!("enter the elements of array and element to be searched");
    //     accept array and search element to be found
    for i in 0..9
    {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        a[i]=input[..].trim().parse::<i32>().unwrap();
    }
    let mut input = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    n=input[..].trim().parse::<i32>().unwrap();
    


    for i in 0..9
    {
         if a[i]==n
         {
            println!("Elment found at {i}th postion");
            flag=1;
         }
    }
    if flag==0
    {
        println!("Element not found");
    }
}


