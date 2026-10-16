.outputs
.dummy Epsilon recvfrom_A0_access_3 recvfrom_A0_syncLogout_5 recvfrom_A2_setup_7 sendto_A0_syncLog_6 sendto_A3_log_8 
.state graph
q0 recvfrom_A2_setup_7 q1
q1 recvfrom_A0_access_3 q1
q1 recvfrom_A0_syncLogout_5 q1c
q1c sendto_A3_log_8 q0b
q0b sendto_A0_syncLog_6 q0
q0 Epsilon qF1148
.marking {q0}
.end
