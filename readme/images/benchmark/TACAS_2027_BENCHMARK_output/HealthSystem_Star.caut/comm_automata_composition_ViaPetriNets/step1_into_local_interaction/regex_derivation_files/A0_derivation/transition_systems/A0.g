.outputs
.dummy Epsilon recvfrom_A1_sendData_1 recvfrom_A2_Ok_5 recvfrom_A2_nok_3 recvfrom_A3_connectOk_13 recvfrom_A3_logCreated_7 recvfrom_A4_fwdOk_9 sendto_A1_notSubscribed_4 sendto_A2_subscribed_2 sendto_A3_account_6 sendto_A3_connect_12 sendto_A4_fwd_8 
.state graph
q0 recvfrom_A1_sendData_1 q1
q1 sendto_A2_subscribed_2 q2
q2 recvfrom_A2_Ok_5 q4
q2 recvfrom_A2_nok_3 q3
q3 sendto_A1_notSubscribed_4 q0
q4 sendto_A3_connect_12 q5
q5 sendto_A3_account_6 q7
q5 recvfrom_A3_logCreated_7 q6
q7 recvfrom_A3_logCreated_7 q8
q6 sendto_A3_account_6 q8
q8 recvfrom_A3_connectOk_13 q9
q9 sendto_A4_fwd_8 q10
q10 recvfrom_A4_fwdOk_9 q0
q0 Epsilon qF90353
.marking {q0}
.end
