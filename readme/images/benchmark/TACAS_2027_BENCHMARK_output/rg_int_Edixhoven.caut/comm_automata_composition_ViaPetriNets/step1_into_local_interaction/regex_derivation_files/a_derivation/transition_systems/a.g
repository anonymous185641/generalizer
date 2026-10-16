.outputs
.dummy Epsilon recvfrom_b_no_4 recvfrom_b_yes_3 sendto_b_bool_2 sendto_b_int_1 
.state graph
q0 sendto_b_int_1 q1
q1 sendto_b_bool_2 q2
q2 recvfrom_b_yes_3 q4
q2 recvfrom_b_no_4 q4
q1 recvfrom_b_yes_3 q3
q1 recvfrom_b_no_4 q3
q3 sendto_b_bool_2 q4
q4 Epsilon qF13512
.marking {q0}
.end
