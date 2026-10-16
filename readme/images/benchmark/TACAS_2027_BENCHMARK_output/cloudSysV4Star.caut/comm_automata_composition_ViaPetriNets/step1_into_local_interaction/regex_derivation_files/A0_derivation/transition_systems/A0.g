.outputs
.dummy Epsilon recvfrom_A1_syncLog_6 recvfrom_A2_syncAccess_3 sendto_A1_access_4 sendto_A1_syncLogout_7 sendto_A2_connect_1 sendto_A2_logout_5 
.state graph
q0 sendto_A2_connect_1 q1
q1 recvfrom_A2_syncAccess_3 q2
q2 sendto_A1_access_4 q2
q2 sendto_A2_logout_5 q3
q3 sendto_A1_syncLogout_7 q4
q4 recvfrom_A1_syncLog_6 q0
q0 Epsilon qF95608
.marking {q0}
.end
