use crate::cli::UuidArguments;
use uuid::Uuid;

/// Format the UUID according to the arguments passed in
fn formatted_uuid(uuid: Uuid, args: &UuidArguments) -> String {
    if args.simple {
        uuid.as_simple().to_string()
    } else if args.braced {
        uuid.as_braced().to_string()
    } else if args.urn {
        uuid.as_urn().to_string()
    } else {
        uuid.as_hyphenated().to_string()
    }
}

/// Print the UUID to stdout
fn print_uuid(uuid: String) {
    println!("{}", uuid)
}

/// Create a uuid_v1 with random bytes
pub fn uuid_v1(args: UuidArguments) {
    (0..args.amount)
        .map(|_| rand::random::<[u8; 6]>())
        .map(|numbers| {
            let uuid = Uuid::now_v1(&numbers);
            formatted_uuid(uuid, &args)
        })
        .for_each(print_uuid)
}

/// Create a uuid_v3 with random bytes
pub fn uuid_v3(args: UuidArguments) {
    (0..args.amount)
        .map(|_| rand::random::<[u8; 6]>())
        .map(|numbers| {
            let uuid = Uuid::new_v3(&Uuid::NAMESPACE_DNS, numbers.to_vec().as_slice());
            formatted_uuid(uuid, &args)
        })
        .for_each(print_uuid);
}

/// Create a uuid_v4
pub fn uuid_v4(args: UuidArguments) {
    (0..args.amount)
        .map(|_| {
            let uuid = Uuid::new_v4();
            formatted_uuid(uuid, &args)
        })
        .for_each(print_uuid)
}

/// Create a uuid_v5 with random bytes
pub fn uuid_v5(args: UuidArguments) {
    (0..args.amount)
        .map(|_| rand::random::<[u8; 6]>())
        .map(|numbers| {
            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, numbers.to_vec().as_slice());
            formatted_uuid(uuid, &args)
        })
        .for_each(print_uuid);
}

/// Create a uuid_v6 with random bytes
pub fn uuid_v6(args: UuidArguments) {
    (0..args.amount)
        .map(|_| rand::random::<[u8; 6]>())
        .map(|numbers| {
            let uuid = Uuid::now_v6(&numbers);
            formatted_uuid(uuid, &args)
        })
        .for_each(print_uuid);
}

/// Create a uuid_v7 with random bytes
pub fn uuid_v7(args: UuidArguments) {
    (0..args.amount)
        .map(|_| {
            let uuid = Uuid::now_v7();
            formatted_uuid(uuid, &args)
        })
        .for_each(print_uuid);
}

/// Create a uuid_v8 with random bytes
pub fn uuid_v8(args: UuidArguments) {
    (0..args.amount)
        .map(|_| rand::random::<[u8; 16]>())
        .map(|numbers| {
            let uuid = Uuid::new_v8(numbers);
            formatted_uuid(uuid, &args)
        })
        .for_each(print_uuid);
}
