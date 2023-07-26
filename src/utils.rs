use std::collections::HashMap;

pub fn oct_to_rwx_permissions(oct: &String) -> String {
    let mut result = String::new();

    let mut oct_to_rwx: HashMap<char, &str> = HashMap::new();
    oct_to_rwx.insert('7', "rwx");
    oct_to_rwx.insert('6', "rw-");
    oct_to_rwx.insert('5', "r-x");
    oct_to_rwx.insert('4', "r--");
    oct_to_rwx.insert('2', "-w-");
    oct_to_rwx.insert('1', "--x");
    oct_to_rwx.insert('0', "---");

    for character in oct.chars() {
        result.push_str(oct_to_rwx.get(&character).unwrap());
    }

    result
}
