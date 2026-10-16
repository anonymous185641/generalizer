.outputs
.dummy Epsilon recvfrom_A0_connect_1 recvfrom_A0_logout_5 sendto_A0_syncAccess_3 sendto_A1_setup_2 
.state graph
q0 recvfrom_A0_connect_1 q1
q1 sendto_A0_syncAccess_3 q2
q2 sendto_A1_setup_2 q4
q1 sendto_A1_setup_2 q3
q3 sendto_A0_syncAccess_3 q4
q4 recvfrom_A0_logout_5 q0
q0 Epsilon qF95655
.marking {q0}
.end
