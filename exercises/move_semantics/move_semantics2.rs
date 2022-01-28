// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let mut vec2 = fill_vec2(&vec0);
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec2.push(88);
    println!("{} has length {} content `{:?}`", "vec2", vec2.len(), vec2);

    let mut vec3 = Vec::new();
    fill_vec3( vec3.as_mut());
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec3", vec3.len(), vec3);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec2(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec3(vec: &mut Vec<i32>)  {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
