mod town;

use town::Town;

fn main() {
    let mut t = Town::new("Dresden", 556780).expect("invalid town values");

    println!("{}", t.info());
    println!("{t}");
    println!("{}", t.name());
    println!("{}", t.residents());

    t.set_name(t.name().to_uppercase())
        .expect("invalid town name");
    t.set_residents(t.residents() + 1)
        .expect("invalid town residents");

    println!("{t}");
}

#[cfg(test)]
mod tests {
    /// T01: `main()` runs without panicking.
    #[test]
    fn main_runs_without_panicking() {
        crate::main();
    }
}
