use std::{cell::RefCell, collections::HashSet, fs, rc::Rc};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let buff: Rc<RefCell<[Option<char>; 3]>> = Rc::new(RefCell::new([None; 3]));
    let buff_clone = buff.clone();
    let mut last: usize = 0;
    for (i, char) in input
        .chars()
        .take_while(move |letter| {
            let buff = buff_clone.borrow();
            buff.contains(&None)
                || buff.contains(&Some(*letter))
                || (HashSet::from(*buff).len() != buff.len())
        })
        .enumerate()
    {
        buff.borrow_mut()[i % 3] = Some(char);
        last = i + 2;
    }

    println!("{}", last);
}
