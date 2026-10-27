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

## Python and Maude installation

You should install python 3.12 or later. 

For anti-unfication we use the [maude](https://github.com/maude-lang/Maude) system, via its python interface.
To install maude execute the following commands:

```
    python3 -m pip install maude
```


If you use Windows, use ```py``` instead of ```python3``` in the above command.

## Petrify installation

To be able to run the benchmarking of the 
composition of communicating automata, download  [Petrify](https://web.archive.org/web/*/https://www.cs.upc.edu/~jordicf/petrify/distrib/*)
and place its executable in the same directory as the executable of Generalizer.
The name of the executable of Petrify should be ```petrify```.

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