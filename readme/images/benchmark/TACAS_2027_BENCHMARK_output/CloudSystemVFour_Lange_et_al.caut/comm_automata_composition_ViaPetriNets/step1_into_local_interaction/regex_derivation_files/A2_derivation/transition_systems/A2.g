.outputs
.dummy Epsilon recvfrom_A0_connect_1 recvfrom_A0_logout_4 sendto_A0_syncAccess_2 sendto_A1_setup_7 
.state graph
q0 recvfrom_A0_connect_1 q1
q1 sendto_A1_setup_7 q2a
q2a sendto_A0_syncAccess_2 q2
q2 recvfrom_A0_logout_4 q0
q0 Epsilon qF1170
.marking {q0}
.end
