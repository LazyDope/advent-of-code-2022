use std::{cell::RefCell, collections::HashSet, fs, rc::Rc};

const MARK_LEN: usize = 13;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let buff: Rc<RefCell<[Option<char>; MARK_LEN]>> = Rc::new(RefCell::new([None; MARK_LEN]));
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
        let mut buff = buff.borrow_mut();
        buff[i % MARK_LEN] = Some(char);
        last = i + 2;
    }

    println!("{}", last);
}
