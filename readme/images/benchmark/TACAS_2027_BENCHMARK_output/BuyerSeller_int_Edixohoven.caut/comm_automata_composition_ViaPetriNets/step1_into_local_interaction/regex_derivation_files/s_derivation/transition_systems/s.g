.outputs
.dummy Epsilon recvfrom_b1_string_1 recvfrom_b2_ok_5 recvfrom_b2_quit_8 recvfrom_b2_string_6 sendto_b1_int_2 sendto_b2_date_7 sendto_b2_int_4 
.state graph
q0 recvfrom_b1_string_1 q1
q1 sendto_b2_int_4 q2
q2 sendto_b1_int_2 q4
q1 sendto_b1_int_2 q3
q3 sendto_b2_int_4 q4
q4 recvfrom_b2_quit_8 q7
q4 recvfrom_b2_ok_5 q5
q5 recvfrom_b2_string_6 q6
q6 sendto_b2_date_7 q7
q7 Epsilon qF11285
.marking {q0}
.end
