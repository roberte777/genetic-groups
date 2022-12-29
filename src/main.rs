use genetic_groups::{class::Student, gp::Population};

fn main() {
    //first thing is to determine how to structure the classes and seed the
    //initial population

    let students = vec![
        Student::new("A".to_string(), None),
        Student::new("B".to_string(), None),
        Student::new("C".to_string(), None),
        Student::new("D".to_string(), None),
        Student::new("E".to_string(), None),
        Student::new("F".to_string(), None),
        Student::new("G".to_string(), None),
        Student::new("H".to_string(), None),
        Student::new("I".to_string(), None),
        Student::new("J".to_string(), None),
        Student::new("K".to_string(), None),
        Student::new("L".to_string(), None),
        Student::new("M".to_string(), None),
        Student::new("N".to_string(), None),
        Student::new("O".to_string(), None),
        Student::new("P".to_string(), None),
        Student::new("Q".to_string(), None),
        Student::new("R".to_string(), None),
        Student::new("S".to_string(), None),
        Student::new("T".to_string(), None),
        Student::new("U".to_string(), None),
        Student::new("V".to_string(), None),
        Student::new("W".to_string(), None),
        Student::new("X".to_string(), None),
        Student::new("Y".to_string(), None),
        Student::new("Z".to_string(), None),
    ];
    let population = Population::new(students, 2);
    for (i, genotype) in population.population.iter().enumerate() {
        println!("Genotype {}", i);
        for (i, grouping) in genotype.groupings.iter().enumerate() {
            println!("Grouping {}", i);
            println!(
                "{:?}",
                grouping
                    .iter()
                    .map(|x| x.name.clone())
                    .collect::<Vec<String>>()
            );
        }
        println!("\n");
    }
}
