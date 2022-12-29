use crate::class::Student;

pub struct Population {
    population: Vec<Genotype>,
}
impl Population {
    pub fn new(students: Vec<Student>, num_groups: usize) -> Population {
        let initial_population = 10;
        let mut population = Vec::with_capacity(initial_population);
        for _ in 0..initial_population {
            population.push(Genotype::rand(&students, &num_groups));
        }
        Population { population }
    }
}
//each genotype is a grouping of students
pub struct Genotype {
    pub fitness: f64,
    pub groupings: Vec<Vec<Student>>,
}
impl Genotype {
    pub fn rand(students: &Vec<Student>, num_groups: &usize) -> Genotype {
        let mut groupings = Vec::with_capacity(*num_groups);
        for _ in 0..*num_groups {
            groupings.push(Vec::new());
        }
        for student in students {
            let group = rand::random::<usize>() % num_groups;
            groupings[group].push(student.clone());
        }
        Genotype {
            fitness: 0.0,
            groupings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_population() {
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
        assert_eq!(population.population.len(), 10);
    }
    #[test]
    fn test_genotype_sizes() {
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
        for genotype in population.population {
            assert_eq!(
                genotype.groupings[0].len() + genotype.groupings[1].len(),
                26
            );
            assert_eq!(genotype.groupings.len(), 2);
        }
    }
}
