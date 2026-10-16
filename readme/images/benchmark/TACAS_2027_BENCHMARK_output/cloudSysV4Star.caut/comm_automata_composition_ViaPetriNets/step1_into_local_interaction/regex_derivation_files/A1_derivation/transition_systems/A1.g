.outputs
.dummy Epsilon recvfrom_A0_access_4 recvfrom_A0_syncLogout_7 recvfrom_A2_setup_2 sendto_A0_syncLog_6 sendto_A3_log_8 
.state graph
q0 recvfrom_A2_setup_2 q1
q1 recvfrom_A0_access_4 q1
q1 recvfrom_A0_syncLogout_7 q2
q2 sendto_A3_log_8 q3
q3 sendto_A0_syncLog_6 q0
q0 Epsilon qF95633
.marking {q0}
.end
