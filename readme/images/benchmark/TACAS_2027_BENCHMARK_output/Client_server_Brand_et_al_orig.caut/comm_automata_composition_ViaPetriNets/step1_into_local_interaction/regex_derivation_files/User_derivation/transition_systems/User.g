.outputs
.dummy Epsilon recvfrom_Server_ACK_2 sendto_Server_ALARM_4 sendto_Server_DONE_3 sendto_Server_REQ_1 
.state graph
ready sendto_Server_REQ_1 wait
wait sendto_Server_DONE_3 ready
ready sendto_Server_ALARM_4 register
register recvfrom_Server_ACK_2 ready
ready Epsilon qF2458
.marking {ready}
.end
