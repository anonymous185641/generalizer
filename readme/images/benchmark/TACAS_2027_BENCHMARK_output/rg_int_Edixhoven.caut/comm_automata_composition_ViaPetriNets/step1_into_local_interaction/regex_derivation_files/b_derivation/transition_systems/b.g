.outputs
.dummy Epsilon recvfrom_a_bool_2 recvfrom_a_int_1 sendto_a_no_4 sendto_a_yes_3 
.state graph
q0 recvfrom_a_int_1 q1
q1 recvfrom_a_bool_2 q2
q2 sendto_a_yes_3 q4
q2 sendto_a_no_4 q4
q1 sendto_a_yes_3 q3
q1 sendto_a_no_4 q3
q3 recvfrom_a_bool_2 q4
q4 Epsilon qF13530
.marking {q0}
.end
