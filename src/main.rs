/// divides two ints
/// takes 5 parameters
/// num: Numerator
/// den: Denominator
/// decimal: should always be false (for recursive function calling)
/// verbose: whether to print all steps

fn div(num:i32,den:i32,level:i32,mut decimal: bool, verbose:bool) -> String{
    let mut answer = String::new();
    if level > 10 {
        return answer;
    }

    if num > den {
        let integers = num/den;
        let rem = num-integers*den;
        if verbose {
            println!("Numerator ({}) is bigger than the Denominator ({}) \n There are {} {} in {} for a total of {} and we have a remainder of {}",num, den, integers, den, num, integers*den, rem);
        }
        answer.push_str(integers.to_string().as_str());
        if rem > 0{
            answer.push_str(div(rem, den, level+1,decimal,verbose).as_str());
        }
    } else if num < den {
        if !decimal {
            answer.push('.');
            decimal = true;
        }
        if verbose {
            println!("Numerator ({}) is smaller than the Denominator({}) so we multiply by 10 and move a decimal place",num,den);
        }
        answer.push_str(div(num*10, den, level+1,decimal,verbose).as_str());
    } else {
        answer.push('1');
    }
    answer
}

fn main() {
    println!("Hello, world!");
    println!("{}",div(8,7,0,false,false));
}
