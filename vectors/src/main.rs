fn main() {

    /*
        Vectors -> only same type
        Vectors are implemented using generics;
     */

    //
    let _v: Vec<i32> = Vec::new();
    //
    let _v = vec![1, 2, 3];
    //
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    //lectura de elementos
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];//acceso indexado
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);//acceso con get
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

/*
     Las borrow checker rules siguen aplicando, dependiendo de si es mut o no.
 */

    let v = vec![100, 32, 57]; //vector inmutable
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57]; //mut
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }


}