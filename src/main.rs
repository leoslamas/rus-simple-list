use list_test::LinkedList;

fn main() {
    let mut my_list = LinkedList::<i32>::new();
    my_list.add(1);
    my_list.add(10);
    my_list.add(100);

    for i in my_list.iter() {
        println!("Borrowing: {}", i)
    }

    let i = my_list.get(1);
    let i1 = my_list.get(2);
    let i2 = my_list.get(1);
    let i3 = my_list.get(0);
    println!("Elem: {:?}\n{:?}\n{:?}\n{:?}", i, i1, i2, i3);

    println!("List: {:?}", my_list);

    let pop = my_list.pop();
    println!("Pop: {}", pop.unwrap());

    for i in my_list.iter() {
        println!("Borrowing: {}", i)
    }

    println!("List: {:?}", my_list);

    my_list.del(1);

    for i in my_list.iter() {
        println!("Borrowing: {}", i)
    }

    println!("List: {:?}", my_list);

}
