.outputs
.dummy Epsilon recvfrom_User_ALARM_4 recvfrom_User_DONE_3 recvfrom_User_REQ_1 sendto_User_ACK_2 
.state graph
idle recvfrom_User_REQ_1 service
service recvfrom_User_DONE_3 idle
idle recvfrom_User_ALARM_4 fault
fault sendto_User_ACK_2 idle
idle Epsilon qF2481
.marking {idle}
.end
