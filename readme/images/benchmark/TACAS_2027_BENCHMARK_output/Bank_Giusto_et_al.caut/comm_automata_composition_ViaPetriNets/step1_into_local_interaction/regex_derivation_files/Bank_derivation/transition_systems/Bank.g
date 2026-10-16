.outputs
.dummy Epsilon recvfrom_Client_infopay_5 recvfrom_Seller_transaction_8 sendto_Client_askpay_4 sendto_Seller_auth_9 sendto_Seller_refusal_10 
.state graph
q0 recvfrom_Seller_transaction_8 q1
q1 sendto_Client_askpay_4 q2
q2 recvfrom_Client_infopay_5 q3
q3 sendto_Seller_auth_9 q0
q3 sendto_Seller_refusal_10 q0
q0 Epsilon qF97368
.marking {q0}
.end
