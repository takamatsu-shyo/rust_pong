use std::collections::HashMap;

pub fn run(){

    let x: i8 = 50;
    let y: i8 = 0;
    let dx: i8 = 1;
    let dy: i8 = 10;
    
    let mut ball = HashMap::from([
        ("x",x),
        ("y",y),
        ("dx",dx),
        ("dy",dy),
    ]);

    for i in 0..100 {

        print!("Time,{} ",i );

        for x in &ball {
            print!("{:?}, ", x); 
        }
        println!("");

        let new_x = ball.get("x").unwrap() + ball.get("dx").unwrap();
        let new_y = ball.get("y").unwrap() + ball.get("dy").unwrap();

        ball.insert("x", new_x );
        ball.insert("y", new_y );

        if new_x.abs() > 99 {
            let new_dx = ball.get("dx").unwrap() * -1;
            ball.insert("dx", new_dx);
        }
 
        if new_y.abs() > 99 {
            let new_dy = ball.get("dy").unwrap() * -1;
            ball.insert("dy", new_dy);
        }
    }
}
