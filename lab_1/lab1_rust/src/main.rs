use std::cmp::Ordering;
use std::io;
use std::env;
use std::io::stdin;

fn get_args()->(f32, f32, f32){
    let mut arg_1:f32 = 0.0;
    let mut arg_2:f32 = 0.0;
    let mut arg_3:f32 = 0.0;
    let mut count = 0;
    let mut args = env::args();
    args.next();
    for str in args{
        let num = str.parse::<f32>();
        match num{
            Ok(val) => {
                match count{
                    0 => {
                        println!("1 argument : {val}");
                        arg_1 = val; 
                        count += 1
                    },
                    1 => {
                        println!("2 argument : {val}");
                        arg_2 = val; 
                        count += 1
                    },
                    2 => {
                        println!("3 argument : {val}");
                        arg_3 = val; 
                        count += 1
                    },
                    _ => return (arg_1, arg_2, arg_3)
                }
            }
            Err(why) =>{
                println!("An Error has occured while reading command line arguments :{why}")
            }
        }
    }

    while count < 3{
        println!("Введите коеффициент!");
        let mut pos_num:String = String::new();
        match io::stdin().read_line(&mut pos_num){
            Ok(_) => {
                let num = pos_num.trim().parse::<f32>();
                match num{
                    Ok(val) =>{
                        match count{
                            0 => {
                                println!("1 argument : {val}");
                                arg_1 = val; 
                                count += 1
                            },
                            1 => {
                                println!("2 argument : {val}");
                                arg_2 = val; 
                                count += 1
                            },
                            2 => {
                                println!("3 argument : {val}");
                                arg_3 = val; 
                                count += 1
                            },
                            _ => return (arg_1, arg_2, arg_3)
                        }
                    }
                    Err(why) =>{
                        println!("An Error has occured while reading line:{why}")
                    }
                }
            }
            Err(_) => {
                println!("Error while getting the input")
            }
        }

    }

    (arg_1, arg_2, arg_3)
}

fn get_root_quadratic((arg_1, arg_2, arg_3) :(f32, f32, f32), number_of_roots:& mut i8) -> (Option<f32>, Option<f32>){

    let discriminant = arg_2 * arg_2 - 4.0 * arg_1 * arg_3;

    if discriminant < 0.0 {
        *number_of_roots = 0;

        return  (None, None);
    }
    if discriminant == 0.0 {
        *number_of_roots = 1;
        if arg_1 == 0.0{
            return  (None, None);
        }
        let root = -arg_2 / (2.0 * arg_1);
        
        return (Some(root), Some(root));
    }
    *number_of_roots = 2;
    if arg_1 == 0.0{
        return  (None, None);
    }
    let sqrt_disc = discriminant.sqrt();
    let root_1 = (-arg_2 + sqrt_disc) / (2.0 * arg_1);
    let root_2 = (-arg_2 - sqrt_disc) / (2.0 * arg_1);

    return (Some(root_1), Some(root_2));
}

fn get_root_biquadratic(number_of_roots: &i8, roots : (Option<f32>, Option<f32>)){
    let mut roots_:Vec<f32>= vec![];
    match number_of_roots{
        0 => {
            println!("Корней нет");
            return;
        }
        1 => {
            if let Some(root) = roots.0 {
                if (root > 0.0){
                    roots_.push(root.sqrt());
                    roots_.push(-root.sqrt());
                }else if (root == 0.0){
                    roots_.push(root.sqrt());
                }
            }else{
                println!("Не биквадратное уравнение");
                return;
            }
        }
        2 =>{
            for i in [roots.0, roots.1].iter(){
                if let Some(root) = i {
                    if (*root > 0.0){
                        roots_.push(root.sqrt());
                        roots_.push(-root.sqrt());
                    }else if (*root == 0.0){
                        roots_.push(root.sqrt());
                    }

                }else{
                    println!("Не биквадратное уравнение");
                    return;
                }
            } 
        }
        _ =>{
            return;;
        }
    }
    if (roots_.len() == 0){
        println!("Нет корней");
    }else{
        println!("Корни биквадратного уравнения:");
        for i in roots_{
            println!("{i}");
        }
    }
}


fn main() {
    let mut num_r = 0;
    let roots = get_root_quadratic(get_args(), &mut num_r);
    get_root_biquadratic(&num_r, roots);
}
