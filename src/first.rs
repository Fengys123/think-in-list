pub mod wrong_list {
    // this is a wrong data structure, not compile
    // pub enum WrongList {
    //     Empty,
    //     Elem(i32, WrongList)
    // }
}

pub mod bad_list {
    pub enum List<T> {
        Empty,
        Elem(T, Box<List<T>>),
    }

    impl<T> List<T> {}
}

pub mod good_list {
    pub struct List {
        head: Link,
    }

    enum Link {
        Empty,
        More(Box<Node>),
    }

    pub struct Node {
        elem: i32,
        next: Link,
    }
}

pub mod better_list {
    pub struct List {
        head: Link,
    }

    pub type Link = Option<Box<Node>>;

    pub struct Node {
        elem: i32,
        next: Link,
    }
}
