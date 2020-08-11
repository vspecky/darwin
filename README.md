# NEAT.rs (Beta)
Implementation of the NEAT Genetic Algorithm in Rust.
<br>
<br>
As of 11th August 2020, the Algorithm is complete, tested and usable. I will try and perform optimizations wherever I can.

## What is NEAT?
NEAT stands for NeuroEvolution of Augmenting Topologies. It is a Genetic Algorithm that evolves
TWEANNs (Topology and Weight Evolving Artificial Neural Networks). This algorithm is a bit different
from other Neural Network evolving Genetic Algorithms because here, the Topology of the ANNs is also
evolved instead of just the weights.  
NEAT derives its inspiration from real world genetics. In genetics, two genotypes cannot cross-over
unless their constituent genes have a certain degree of similarity. NEAT reproduces this logic by
having a mechanism that keeps track of the history of Innovation (or evolution) over time so we
can have a measure of how similar two members of the population (here, Neural Networks) are and also
what similarities they share. This allows us to effectively perform crossover between two Neural
Networks no matter what the topology is. Additionally, the similarity measure also allows us to 
perform speciation, so we can group up similar neural networks and have them compete within their
own niche. More Info can be gleaned by reading the original paper. (Link is down below)  

## Useful/Interesting Videos Showcasing NEAT
- [SethBling's MarI/O](https://www.youtube.com/watch?v=qv6UVOQ0F44)  
- [FlapPyBi/o](https://www.youtube.com/watch?v=H4WnRLEG73Q)  
- [Self Driving Car in Unity](https://www.youtube.com/watch?v=2bW9CdFcaUI)  
- [AI Learns to Drive by Code Bullet](https://www.youtube.com/watch?v=r428O_CMcpI)

## Reference
[K. Stanley, R. Miikkulainen (2002) 'Evolving Neural Networks through Augmenting Topologies'](http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf)
