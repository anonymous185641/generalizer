### Drawings of the input automata

The automata `A0`, `A1`,`A2`,`A3`,`A4`,`A5` are drawn as follows( from left to right and top to bottom):


|                          Local automatons                           |  |
|:-------------------------------------------------------------------:| :---: |
| <img src="../images/caut_comp/A0.png" alt="Automaton A0" width="400"/> | <img src="../images/caut_comp/A1.png" alt="Automaton A1" width="400"/> |
| <img src="../images/caut_comp/A2.png" alt="Automaton A2" width="400"/> | <img src="../images/caut_comp/A3.png" alt="Automaton A3" width="400"/> |
| <img src="../images/caut_comp/A4.png" alt="Automaton A4" width="400"/> | <img src="../images/caut_comp/A5.png" alt="Automaton A5" width="400"/> |


### Result with the method Petrify+Rewriting

By executing the command:

```
$ generalizer caut_composition caut_file.caut -p -v
```

we obtain the following results:


#### Local interactions

| Local Interactions | |
| :---: | :---: |
| <img src="../images/caut_comp/via_petrify/HSS_i0.png" alt="Local interaction of A0" width="80"/> | <img src="../images/caut_comp/via_petrify/HSS_i1.png" alt="Local interaction of A1" width="80"/> |
| <img src="../images/caut_comp/via_petrify/HSS_i2.png" alt="Local interaction of A2" width="80"/> | <img src="../images/caut_comp/via_petrify/HSS_i3.png" alt="Local interaction of A3" width="80"/> |
| <img src="../images/caut_comp/via_petrify/HSS_i4.png" alt="Local interaction of A4" width="80"/> | <img src="../images/caut_comp/via_petrify/HSS_i5.png" alt="Local interaction of A5" width="80"/> |

#### Global interaction

<img src="../images/caut_comp/via_petrify/HSS.png" alt="Resulting global interaction" width="300"/>

### Result with the method via Brzozowski's algorithm

By executing the command:

```
$ generalizer caut_composition caut_file.caut -b -v
```

we obtain the following results:


#### Local interactions

| Local Interactions | |
| :---: | :---: |
| <img src="../images/caut_comp/via_Brzozowski/HSS_b_i0.png" alt="Local interaction of A0" width="80"/> | <img src="../images/caut_comp/via_Brzozowski/HSS_b_i1.png" alt="Local interaction of A1" width="80"/> |
| <img src="../images/caut_comp/via_Brzozowski/HSS_b_i2.png" alt="Local interaction of A2" width="80"/> | <img src="../images/caut_comp/via_Brzozowski/HSS_b_i3.png" alt="Local interaction of A3" width="80"/> |
| <img src="../images/caut_comp/via_Brzozowski/HSS_b_i4.png" alt="Local interaction of A4" width="80"/> | <img src="../images/caut_comp/via_Brzozowski/HSS_b_i5.png" alt="Local interaction of A5" width="80"/> |

#### Global interaction

<img src="../images/caut_comp/via_Brzozowski/HSS_b.png" alt="Resulting global interaction" width="300"/>