.outputs
.dummy Epsilon recvfrom_Client_acks_3 recvfrom_Client_req_1 sendto_Client_res_2 sendto_Database_logs_4 
.state graph
q0 recvfrom_Client_req_1 q1
q1 sendto_Client_res_2 q2
q2 recvfrom_Client_acks_3 q3
q3 sendto_Database_logs_4 q0
q0 Epsilon qF99471
.marking {q0}
.end
