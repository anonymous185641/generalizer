.outputs
.dummy Epsilon recvfrom_A1_price_2 sendto_A1_hag_1 sendto_A1_happy_3 sendto_A2_info_4 
.state graph
q0 sendto_A1_hag_1 q1
q1 recvfrom_A1_price_2 q0
q0 sendto_A1_happy_3 q2
q2 sendto_A2_info_4 q4
q0 sendto_A2_info_4 q3
q3 sendto_A1_happy_3 q4
q4 Epsilon qF13026
.marking {q0}
.end
