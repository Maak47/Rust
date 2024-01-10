fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);
    let a_is_some = a.is_some(); //bool to check whether the variable is Option(some) or none => True
    dbg!(a_is_some);
    let a_is_none = a.is_none(); //bool to check whether the variable is Option(none) or some => false
    dbg!(a_is_none);
    let a_mapped = a.map(|num| num + 1); // adds 1 to the value of Some() or doesnt do anything if the value is None
    dbg!(a_mapped);
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);
    let unwrapped = a.unwrap_or_else(|| 0); // if their is no initial value, the or_else assigns the default value of 0 to it
    dbg!(unwrapped);
}
