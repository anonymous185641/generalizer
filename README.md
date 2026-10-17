# Generalizer



Generalizer is an implementation for the composition
of local interaction models of communicating automata into
global interaction models using anti-unification techniques.




## How to build


You can build the project with cargo using

```
cargo build --release
```


An executable will be created in "./target/release".


# Composition of communicating automata

The subcommand for the inference of global interaction models is:

- `caut_composition`

`caut` standing for "communicating automata".

[Composition of systems of communicating automata](readme/communicating-automata-composition.md)



# Benchmarking of the composition of communicating automata

We provide a benchmarking of the composition of communicating automata
by comparing two different approaches: one based on the rewriting of 
Petri nets and the other based on the Brzozowski's algorithm for the derivation
of regular expressions.

[Benchmarking of the composition of communicating automata](readme/benchmark-caut-comp.md)

# License
This project is licensed under the GNU GPL version 3 or later (GPL-3.0-or-later).  
See the LICENSE file for details.