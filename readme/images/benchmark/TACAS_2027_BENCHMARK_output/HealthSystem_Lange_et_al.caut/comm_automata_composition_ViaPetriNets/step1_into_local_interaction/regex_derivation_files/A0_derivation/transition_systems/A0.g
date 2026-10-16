.outputs
.dummy Epsilon recvfrom_A1_sendData_1 recvfrom_A2_nok_3 recvfrom_A2_ok_5 recvfrom_A3_logCreated_7 recvfrom_A4_fwdOk_9 sendto_A1_notSubscribed_4 sendto_A2_subscribed_2 sendto_A3_account_6 sendto_A4_fwd_8 
.state graph
s1 recvfrom_A1_sendData_1 s2
s2 sendto_A2_subscribed_2 s3
s3 recvfrom_A2_nok_3 s6
s6 sendto_A1_notSubscribed_4 s1
s3 recvfrom_A2_ok_5 s4
s4 sendto_A3_account_6 s5
s5 recvfrom_A3_logCreated_7 s7
s7 sendto_A4_fwd_8 s8
s8 recvfrom_A4_fwdOk_9 s1
s1 Epsilon qF17039
.marking {s1}
.end
