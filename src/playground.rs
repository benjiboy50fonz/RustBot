struct Cat {
    age: i8,
    weight: f32,
    male: bool
}

struct vv (i8, i8, i8);

struct open<X, Z> {
    x: X,
    y: Z
}

fn main() {

    let mut ringo = Cat {age: 2, weight: 5.9, male: true };

    ringo.weight = 6.2;

    let v = vv(5,6,7);

    let u = vv;

    let o = open {x: 4, y: 6.7};

    let k = open {x: true, y: "lol"};





    let x: f32 = {
        let y = 0.0;
        let d = 3.3;
        
        d + y
    };

    println!("{}", x);
    t(5);

    let t2: fn(i8) = t;

    t2(6);

    let temp = |x| -> f32 {
        (x * x) as f32
    }(7);

    let g: [i32; 4] = [4,3,2,1];

    println!("{}", temp);
    
    let mut a: Vec<i32> = Vec::new();
    let mut b = vec![1,2,3];
    let mut c = vec![5; 10];
    let mut d: Vec<i32> = vec![];

    c[3] = 6;
    a.push(6);
    a.push(9);

    for i in 0..10 {
        b.push(i);
    }

    for i in 0..10 { d.push(i) }

    for i in &mut d {
        *i += 2;
    }

}

pub fn t(a: i8) {
    println!("{}", a);
}

pub fn hello() -> String {   
    ("Hello, world!").to_string()
}

#[cfg(test)]
mod tests {
    use super::hello;

    fn test_hello() {
        assert_eq!(hello(), "Hello, world!")
    }
}
