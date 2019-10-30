fn add_elems(s: &[i32]) -> Option<i32> {
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}

fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);

    let mut o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        assert_eq!(s, 20);
    }

    let mut o2 = Some(String::from("Hello"));
    assert_eq!(o2.unwrap(), "Hello");

    o2 = None;
    assert_eq!(
        o2.unwrap_or_else(|| String::from("o2 is none")),
        "o2 is none"
    );

    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));

    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);

    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
            .and_then(|n| if n >= 200 { Some(n) } else { None }),
        None
    );

    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
    assert_eq!(add_elems(&[1]), None);

    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);

    // tuple-like struct
    struct UserName(String);
    struct Id(u64); // Instead of `type Id = u64` to avoid a error due to mistaking position
    struct Timestamp(u64);
    type User = (Id, UserName, Timestamp);

    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
        (id, name, created)
    }

    let id = Id(400);
    let now = Timestamp(4567890123);
    let _ = new_user(UserName(String::from("shunp")), id, now); // force me to protect the order

    type UserNameType = String;
    #[derive(Debug)]
    enum Task {
        Open,
        AssignedTo(UserNameType),
        Working {
            assignee: UserNameType,
            remaining_hours: u16,
        },
        Done,
    }

    use Task::*; // Declare instead of `Task::AssignedTo`
    let tasks = vec![
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => {
                println!("the task {} is assigned by {}", i, assignee)
            }
            Working { assignee, remaining_hours } => {
                println!("the task {} is being done by {}. The remain is {} hours", i, assignee, remaining_hours)
            }
            _ => println!("the task {} is the status {:?}", i, task)
        }
    }
}
