.outputs
.dummy Epsilon recvfrom_A0_account_6 recvfrom_A0_connect_12 sendto_A0_connectOk_13 sendto_A0_logCreated_7 
.state graph
q0 recvfrom_A0_connect_12 q1
q1 recvfrom_A0_account_6 q2
q1 sendto_A0_logCreated_7 q3
q2 sendto_A0_logCreated_7 q4
q3 recvfrom_A0_account_6 q4
q4 sendto_A0_connectOk_13 q0
q0 Epsilon qF90418
.marking {q0}
.end
