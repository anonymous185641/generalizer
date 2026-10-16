.outputs
.dummy Epsilon recvfrom_A1_itemToBeFiltered_3 recvfrom_A1_noMoreItems_2 sendto_A1_Ok_5 sendto_A1_newFilterRequest_1 sendto_A1_remove_4 
.state graph
q0 sendto_A1_newFilterRequest_1 q1
q1 recvfrom_A1_noMoreItems_2 q0
q1 recvfrom_A1_itemToBeFiltered_3 q2
q2 sendto_A1_Ok_5 q3
q2 sendto_A1_remove_4 q4
q3 sendto_A1_remove_4 q1
q4 sendto_A1_Ok_5 q1
q0 Epsilon qF3341
.marking {q0}
.end
