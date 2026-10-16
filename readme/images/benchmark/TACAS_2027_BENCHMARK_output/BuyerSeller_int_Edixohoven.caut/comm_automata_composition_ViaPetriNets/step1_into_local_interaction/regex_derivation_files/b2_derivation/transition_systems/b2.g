.outputs
.dummy Epsilon recvfrom_b1_int_3 recvfrom_s_date_7 recvfrom_s_int_4 sendto_s_ok_5 sendto_s_quit_8 sendto_s_string_6 
.state graph
q0 recvfrom_s_int_4 q1
q0 recvfrom_b1_int_3 q2
q1 recvfrom_b1_int_3 q3
q2 recvfrom_s_int_4 q3
q3 sendto_s_ok_5 q4
q3 sendto_s_quit_8 q6
q4 sendto_s_string_6 q5
q5 recvfrom_s_date_7 q6
q6 Epsilon qF11262
.marking {q0}
.end
