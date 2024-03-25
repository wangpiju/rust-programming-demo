
pub fn array_demo()-> [bool; 20]  {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16]; 
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    let mut sieve: [bool; 20] = [true; 20];
    for i in 2..10 {
        if sieve[i] {
            let mut j = i * i; 
            while j < 20 {
                // println!("{}", j);
                sieve[j] = false;
                j += i; 
            }
        } 
    }
    sieve

}