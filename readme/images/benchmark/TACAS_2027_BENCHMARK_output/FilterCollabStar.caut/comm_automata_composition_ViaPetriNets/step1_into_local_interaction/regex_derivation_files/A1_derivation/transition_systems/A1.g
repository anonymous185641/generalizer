.outputs
.dummy Epsilon recvfrom_A0_Ok_5 recvfrom_A0_newFilterRequest_1 recvfrom_A0_remove_4 sendto_A0_itemToBeFiltered_3 sendto_A0_noMoreItems_2 
.state graph
q0 recvfrom_A0_newFilterRequest_1 q1
q1 sendto_A0_noMoreItems_2 q0
q1 sendto_A0_itemToBeFiltered_3 q2
q2 recvfrom_A0_Ok_5 q3
q2 recvfrom_A0_remove_4 q4
q3 recvfrom_A0_remove_4 q1
q4 recvfrom_A0_Ok_5 q1
q0 Epsilon qF3364
.marking {q0}
.end
