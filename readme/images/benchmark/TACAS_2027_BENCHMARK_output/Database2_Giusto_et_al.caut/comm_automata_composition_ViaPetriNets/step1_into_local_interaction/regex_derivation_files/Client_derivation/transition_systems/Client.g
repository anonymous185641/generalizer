.outputs
.dummy Epsilon recvfrom_Database_ackd_5 recvfrom_Server_res_2 sendto_Database_logc_6 sendto_Server_acks_3 sendto_Server_req_1 
.state graph
q0 sendto_Server_req_1 q1
q1 recvfrom_Server_res_2 q2
q2 sendto_Server_acks_3 q0
q3 recvfrom_Database_ackd_5 q0
q0 sendto_Database_logc_6 q3
q0 Epsilon qF99488
.marking {q0}
.end
