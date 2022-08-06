#![allow(non_snake_case)]

fn main() {
    shapes();
    factorials(56.0);
}

fn shapes () {

    // example 1:
    // Task: find the area of a circle

    // solution: pi times by radius squared

    let pi: f64 = 3.141592;
    let radius: f64 = 1.2345; // radius of circle
    let cirarea: f64 = pi * radius * radius; // multiply pi by radius squared
    println!("Area: {}", cirarea);


    // example 2:
    // Task: find the hypotenuse of a right-angled triangle (the missing value of a side)

    // solution: a squared plus b squared equals c squared

    let a: f64 = 1.2345; // side a
    let b: f64 = 2.3456; // side b
    let mut c: f64; // side c (no value, mutable)
    c = a * a + b * b; // a squared plus b squared
    c = c * c; // c squared (final answer)
    println!("Side C: {}", c);


    // example 3:
    // Task: Find the area of triangle

    // solution: height multiplied by width (base), then devide by 2

    let height: f64 = 1.2345;
    let width: f64 = 1.2345;
    let triarea: f64 = height * width / 2.0;
    println!("Area: {}", triarea);


    // example 4:
    // Task: Find the missing angle in a triangle

    // solution: angle a plus angle b, subtract from 180

    let angleA: f64 = 57.65;
    let angleB: f64 = 23.35;
    let angleC: f64 = angleA + angleB - 180.0;
    println!("Angle C: {}", angleC);


    /*
        Secion 2 of shapes; trigonometry
    */


    // example 5:
    // Task: Determine wheather to use sine, cosine or tangent depending on the input variables
    // to find the missing value in a triangle. Let's assume that the angle is located in the top
    // right of a right angled, equalatrial triangle.

    // solution: Sin * opposite / hypotonouse, Cos * adjacent / hypotonouse, Tan * Opposite / adjacent

    let SideA: f64 = 8.0;
    let SideB: f64 = 10.0;
    let SideC: f64 = 14.0;
    let AnglePlacementPos: u32 = 1;
    let mut Answer: f64 = 0.0;

    if AnglePlacementPos == 0 {
        // use Cos
        Answer = SideB / SideC;
        Answer = Answer.cos();
    }
    else if AnglePlacementPos == 1 {
        // use Tan
        Answer = SideB / SideA;
        Answer = Answer.tan();
    }
    else if AnglePlacementPos == 2 {
        // use Sin
        Answer = SideA / SideC;
        Answer = Answer.sin();
    }
    println!("Trig Answer: {}", Answer);
    
}


    /*
    
        Factorials
        In this section, I'll be using my own custom, simple algorithm
        to find the prime factors of an input cariable.

        The primary focus of this exercise to to consolidate my understanding
        of how to use mathematics in Rust and to illustrate the best possible
        sollution for optimisation.

    */


static mut i_counter: u64 = 0;
static mut i_answer: [u64; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]; // supports up to 20 factorials
static mut i_OriginalNum: f64 = 0.0;

fn factorials (input: f64) {
    unsafe {
        // save the original number
        if i_counter == 0 {
            i_OriginalNum = input;
        }
        if i_counter < 19 {
            let tempnum: f64 = input / 2.0;
            if tempnum.fract() == 0.0 {
                // is a factorial, add it to the array
                i_answer[i_counter as usize] = (tempnum * 100.0).round() as u64;
                i_counter = i_counter + 1;
            }
            else {
                i_counter = i_counter + 1; 
                // is not a factorial, pass back through
                factorials(tempnum);
            } 
        }
        else {
            // maximum array lenth reached!
            println!("Maximum amount of factorials reached!");
            println!("Recorded factorials of {} : ", i_OriginalNum);
            println!("{:?}", i_answer);        
        }
    }

}