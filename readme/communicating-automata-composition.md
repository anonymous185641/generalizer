

## Communicating Automata Composition

The command `caut_composition` takes a `.caut` file as input.

A folder is created containing the results.

An example of application is:

```
$ generalizer caut_composition caut_file.caut
```
The tool Petrify must be in the same folder as the executable.

These are the links for the downloading the tool `Petrify`: [Petrify](https://web.archive.org/web/*/https://www.cs.upc.edu/~jordicf/petrify/distrib/*)

The command accepts the following options:
- `-b` to use the algorithm of Brzozowski for the derivation of regular expressions.
- `-p` for using the technique via `Petrify` and Petri nets rewriting for the derivation of regular expressions with interleaving.
- `-a` to use both the algorithm of Brzozowski and the technique via `Petrify` for the derivation of regular expressions.
- `-v` for the use of equational variants for anti-unification.


By default, the Brzozowski algorithm is used for the derivation of regular expressions.


## .caut file format


We will use the example [HealthSystem_Star.caut](../examples/communicating_automata/HealthSystem_Star.caut) to illustrate the syntax.

The content of the file is:

```
@automata{A0;A1;A2;A3;A4;A5}
@messages{account;connect;connectOk;fwd;fwdOk;helpReq;logCreated;nok;notSubscribed;Ok;provideService;sendData;subscribed}
@gates{1;2;3;4;5;6;7;8;9;10;11;12;13}

.automaton A0
q0 recvfrom(A1,sendData)[1] q1
q1 sendto(A2,subscribed)[2] q2
q2 recvfrom(A2,Ok)[5] q4
q2 recvfrom(A2,nok)[3] q3
q3 sendto(A1,notSubscribed)[4] q0
q4 sendto(A3,connect)[12] q5
q5 sendto(A3,account)[6] q7
q5 recvfrom(A3,logCreated)[7] q6
q7 recvfrom(A3,logCreated)[7] q8
q6 sendto(A3,account)[6] q8
q8 recvfrom(A3,connectOk)[13] q9
q9 sendto(A4,fwd)[8] q10
q10 recvfrom(A4,fwdOk)[9] q0
.initial state {q0}
.final states {q0}
.end

.automaton A1
q0 sendto(A0,sendData)[1] q1
q1 recvfrom(A0,notSubscribed)[4] q0
q1 recvfrom(A5,provideService)[10] q0
.initial state {q0}
.final states {q0}
.end

.automaton A2
q0 recvfrom(A0,subscribed)[2] q1
q1 sendto(A0,nok)[3] q0
q1 sendto(A0,Ok)[5] q0
.initial state {q0}
.final states {q0}
.end

.automaton A3
q0 recvfrom(A0,connect)[12] q1
q1 recvfrom(A0,account)[6] q2
q1 sendto(A0,logCreated)[7] q3
q2 sendto(A0,logCreated)[7] q4
q3 recvfrom(A0,account)[6] q4
q4 sendto(A0,connectOk)[13] q0
.initial state {q0}
.final states {q0}
.end

.automaton A4
q0 recvfrom(A0,fwd)[8] q1 
q1 sendto(A0,fwdOk)[9] q2
q2 sendto(A5,helpReq)[11] q0
.initial state {q0}
.final states {q0}
.end

.automaton A5
q0 recvfrom(A4,helpReq)[11] q1
q1 sendto(A1,provideService)[10] q0
.initial state {q0}
.final states {q0}
.end
```
The file begins with a declaration of the automata, messages and gates of the system.

Then, each automaton of the system is detailed.
For each automaton, the `.automaton` keyword is followed by the name of the automaton.
The transitions are provided in the format 'q1 a q2', where q1 is the starting
state, 'a' an action and q2 the destination state.

An action is of one of the following types:

- an emission `sendto(A,m)[g]`, where `A` is the destination automaton, `m` the message transmitted and `g` the associated gate.
- a reception `recvfrom(A,m)[g]` where `A` is the source automaton, `m` the message received and `g` the associated gate.

There can only be one transition per line.

The initial states and final states of the automaton 
are provided with the keyword `.initial state` and `.final states` respectively.

The keyword `.end` marks the end of the description of each automaton.

## Output

### Drawings of the input automata

The automata `A0`, `A1`,`A2`,`A3`,`A4`,`A5` are drawn as follows( from left to right and top to bottom):

<img src="images/caut_comp/A0.png" alt="Automaton A0" width="500"/>
<img src="images/caut_comp/A1.png" alt="drawing" width="400"/>
<img src="images/caut_comp/A2.png" alt="drawing" width="400"/>
<img src="images/caut_comp/A3.png" alt="drawing" width="400"/>
<img src="images/caut_comp/A4.png" alt="drawing" width="400"/>
<img src="images/caut_comp/A5.png" alt="drawing" width="400"/>


### Result with the method Petrify+Rewriting

By executing the command:

```
$ generalizer caut_composition caut_file.caut -p -v
```

we obtain the following results:


#### Local interactions

<img src="images/caut_comp/via_petrify/HSS_i0.png" alt="Local interaction of A0" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_petrify/HSS_i1.png" alt="Local interaction of A1" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_petrify/HSS_i2.png" alt="Local interaction of A2" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_petrify/HSS_i3.png" alt="Local interaction of A3" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_petrify/HSS_i4.png" alt="Local interaction of A4" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_petrify/HSS_i5.png" alt="Local interaction of A5" width="80"/>

#### Global interaction

<img src="images/caut_comp/via_petrify/HSS.png" alt="Resulting global interaction" width="300"/>

### Result with the method via Brzozowski's algorithm

By executing the command:

```
$ generalizer caut_composition caut_file.caut -b -v
```

we obtain the following results:


#### Local interactions

<img src="images/caut_comp/via_Brzozowski/HSS_b_i0.png" alt="Local interaction of A0" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_Brzozowski/HSS_b_i1.png" alt="Local interaction of A1" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_Brzozowski/HSS_b_i2.png" alt="Local interaction of A2" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_Brzozowski/HSS_b_i3.png" alt="Local interaction of A3" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_Brzozowski/HSS_b_i4.png" alt="Local interaction of A4" width="80"/>
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="images/caut_comp/via_Brzozowski/HSS_b_i5.png" alt="Local interaction of A5" width="80"/>

#### Global interaction

<img src="images/caut_comp/via_Brzozowski/HSS_b.png" alt="Resulting global interaction" width="300"/>