.outputs
.dummy Epsilon recvfrom_dev_hbt_6 recvfrom_dev_sta_3 recvfrom_srv_not_1 sendto_srv_rep_4 
.state graph
q0 recvfrom_srv_not_1 q1
q1 recvfrom_dev_sta_3 q2
q2 sendto_srv_rep_4 q1
q1 recvfrom_dev_hbt_6 q1
q1 Epsilon qF106706
.marking {q0}
.end
