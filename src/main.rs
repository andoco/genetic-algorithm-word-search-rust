use rand::{seq::SliceRandom, Rng};

fn main() {
    let genome = r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOP QRSTUVWXYZ 1234567890, .-;:_!"#%&/()=?@${[]}"##.as_bytes().to_vec();
    let target = "Rust is really really great".as_bytes().to_vec();
    let chromosome_size = target.len();
    let population_size = 100;
    let mut population = vec![];

    let mut rng = rand::thread_rng();

    for _ in 0..population_size {
        let chromosome: Chromosome = genome
            .choose_multiple(&mut rng, chromosome_size)
            .cloned()
            .collect();
        population.push(chromosome);
    }

    let mut i = 0;
    let max_generations = 100_000;

    loop {
        population.sort_by_key(|c| score_fitness(&c, &target));

        let best = population.first().unwrap();

        println!("Generation {}: {}", i, format(best));

        if check_match(best, &target) {
            println!("Found target match");
            break;
        }

        let mut new_generation: Vec<Chromosome> = vec![];

        let top_10_perc: Vec<_> = select_top_perc(&population, 0.1).iter().cloned().collect();
        new_generation.extend(top_10_perc.iter().cloned().cloned());

        let top_50_perc: Vec<_> = select_top_perc(&population, 0.5).iter().cloned().collect();

        for _ in 0..population_size - new_generation.len() {
            let p1 = top_50_perc.choose(&mut rng).unwrap();
            let p2 = top_50_perc.choose(&mut rng).unwrap();
            let child = crossover(p1, p2, &mut rng);
            let child = mutate(&child, &genome, &mut rng);
            new_generation.push(child);
        }

        population = new_generation;

        for _ in 0..(population_size - population.len()) {
            let chromosome: Chromosome = genome
                .choose_multiple(&mut rng, chromosome_size)
                .cloned()
                .collect();
            population.push(chromosome);
        }

        i += 1;

        if i == max_generations {
            println!("Limit reached");
            break;
        }
    }
}

type Chromosome = Vec<u8>;

fn score_fitness(value: &Chromosome, target: &Chromosome) -> usize {
    let mut score = 0;

    for i in 0..target.len() {
        match (value.get(i), target.get(i)) {
            (Some(a), Some(b)) if *a == *b => {}
            _ => score += 1,
        }
    }

    score
}

fn crossover<R>(value: &Chromosome, other: &Chromosome, rng: &mut R) -> Chromosome
where
    R: Rng + ?Sized,
{
    let mut offspring = vec![];

    for i in 0..value.len() {
        if rng.gen_bool(0.5) {
            offspring.push(*value.get(i).unwrap());
        } else {
            offspring.push(*other.get(i).unwrap());
        }
    }

    offspring
}

fn mutate<R>(value: &Chromosome, genome: &Vec<u8>, rng: &mut R) -> Chromosome
where
    R: Rng + ?Sized,
{
    let mut new_value = vec![];

    for val in value.iter() {
        if rng.gen_bool(0.1) {
            new_value.push(*genome.choose(rng).unwrap());
        } else {
            new_value.push(*val);
        }
    }

    new_value
}

fn check_match(a: &Chromosome, b: &Chromosome) -> bool {
    a.iter().zip(b).filter(|(a, b)| **a == **b).count() == b.len()
}

fn select_top_perc(population: &Vec<Chromosome>, perc: f32) -> Vec<&Chromosome> {
    let n = (population.len() as f32 * perc) as usize;
    population.iter().take(n).collect()
}

fn print_population(population: &Vec<Chromosome>) {
    population
        .iter()
        .for_each(|c| println!("{:?}", String::from_utf8(c.clone()).unwrap()));
}

fn format(chromosome: &Chromosome) -> String {
    String::from_utf8(chromosome.clone()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_fitness() {
        for (value, target, expected_score, msg) in [
            ("AAA", "BBB", 3, "none equal"),
            ("ABA", "BBB", 2, "one equal"),
            ("ABB", "BBB", 1, "two equal"),
            ("BBB", "BBB", 0, "all equal"),
        ] {
            let score = score_fitness(&value.as_bytes().to_vec(), &target.as_bytes().to_vec());
            assert_eq!(expected_score, score, "{}", msg);
        }
    }
}
