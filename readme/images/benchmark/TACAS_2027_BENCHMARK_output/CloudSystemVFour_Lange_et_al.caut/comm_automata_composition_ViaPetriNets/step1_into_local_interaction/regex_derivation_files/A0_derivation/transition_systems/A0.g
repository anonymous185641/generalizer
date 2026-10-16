.outputs
.dummy Epsilon recvfrom_A1_syncLog_6 recvfrom_A2_syncAccess_2 sendto_A1_access_3 sendto_A1_syncLogout_5 sendto_A2_connect_1 sendto_A2_logout_4 
.state graph
q0 sendto_A2_connect_1 q1a
q1a recvfrom_A2_syncAccess_2 q1
q1 sendto_A1_access_3 q1
q1 sendto_A2_logout_4 q0c
q0c sendto_A1_syncLogout_5 q0b
q0b recvfrom_A1_syncLog_6 q0
q0 Epsilon qF1123
.marking {q0}
.end
