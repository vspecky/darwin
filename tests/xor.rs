/**
 * Here we use NEAT to evolve a Neural Network that satisfies the XOR problem.
 * An XOR Gate is a logic gate in electronics that performs exclusive or, i.e
 * the output is 1 when the input bits differ and 0 when they are the same.
 * This relation is non-linear hence it requires a non-linear solution, hence we need
 * to evolve a Neural Network.
 *
 * XOR Truth Table :-
 * Input   Input   Output
 *   0       0       0
 *   0       1       1
 *   1       0       1
 *   1       1       0
 */
// Bring the required structs into scope
extern crate neat_rs;
use neat_rs::Population;
use neat_rs::Settings;

#[test]
fn xor() {
    // Here we initialize the settings for our NEAT Population. The Settings Struct's 'new'
    // function takes in three arguments (Input Size, Output Size, Population Size) and is based
    // upon a builder pattern so we can edit the default options via functions. For example here we
    // use the 'conn_mut_rate' function to set the Connection Mutation Rate to 5%.
    let sets = Settings::new(2, 1, 150).conn_mut_rate(0.05);

    // Here we Initialize the initial population by calling the Population Struct's 'new' method
    // and passing our settings to it.
    let mut pop = Population::new(sets);

    // We create a mutable variable for the best fitness ever found. The Fitness of a member of the
    // population is proportional to how well it can solve the given problem. After every
    // generation, the best fitness of the population is updated. This variable serves no real
    // purpose in this example since we will run NEAT for exactly 150 generations. However it is
    // quite useful when you want to continue running the algorithm until a certain fitness is
    // reached (while best_fitness < some_fitness {})
    let mut best_fitness = 0.;

    // We shall run the algorithm for exactly 150 generations
    for _ in 0..150 {
        // The 'get_citizens' method gives us a Vector containing mutable references to all members
        // of the population
        let genomes = pop.get_citizens();

        // We iterate over the Vector of mutable references, consuming the references as we go
        for g in genomes {
            // Every Citizen/Genome has a 'feed_forward' method that takes in some input, forward
            // propagates it through its neural network and returns a Result. The Result will be an
            // Err when the length of the inputs provided does not match the length of the input
            // size of the genome. The result will be a Ok(output) otherwise where output is a
            // Vec<f64> containing all the output values
            let o1 = g.feed_forward(&vec![0., 0.]).unwrap();

            // We use the 'add_fitness' method of the Genome to add to its fitness. Using the
            // method is necessary since we do not want the fitness value to be negative (which
            // would break the algorithm). The function ensures this.

            // In this case, the output is expected to be zero, hence we add (1 - output) to the
            // fitness such that the closer the output is to 0, the closer the added fitness value
            // is to 1.
            g.add_fitness(1. - o1[0]);

            // We repeat the same step for all possible XOR inputs, assigning the fitness as
            // required
            let o2 = g.feed_forward(&vec![0., 1.]).unwrap();

            // In this case, the output is expected to be 1 hence we add the output as it is to the
            // fitness such that the closer the output is to 1, the better.
            g.add_fitness(o2[0]);

            let o3 = g.feed_forward(&vec![1., 0.]).unwrap();
            g.add_fitness(o3[0]);

            let o4 = g.feed_forward(&vec![1., 1.]).unwrap();
            g.add_fitness(1. - o4[0]);
        }

        // Once we have evaluated all citizens and assigned fitnesses to all of them, we call the
        // 'next_generation' method on the Population. With this, the Population will perform
        // natural selection and give birth to the next generation.
        pop.next_generation();

        // Here we assign the best fitness of this generation to the mutable variable we created
        // earlier.
        best_fitness = pop.best_fitness;
    }

    assert!(best_fitness > 3.0)
}
