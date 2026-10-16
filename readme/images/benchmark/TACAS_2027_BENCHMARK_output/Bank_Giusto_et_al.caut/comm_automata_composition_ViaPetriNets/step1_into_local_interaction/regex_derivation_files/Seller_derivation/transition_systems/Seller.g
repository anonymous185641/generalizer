.outputs
.dummy Epsilon recvfrom_Bank_auth_9 recvfrom_Bank_refusal_10 recvfrom_Client_askprice_1 recvfrom_Client_buy_3 sendto_Bank_transaction_8 sendto_Client_cancel_6 sendto_Client_conf_7 sendto_Client_price_2 
.state graph
q0 recvfrom_Client_askprice_1 q1
q1 sendto_Client_price_2 q0
q0 recvfrom_Client_buy_3 q2
q2 sendto_Bank_transaction_8 q3
q3 recvfrom_Bank_auth_9 q4
q4 sendto_Client_conf_7 q0
q3 recvfrom_Bank_refusal_10 q5
q5 sendto_Client_cancel_6 q0
q0 Epsilon qF97296
.marking {q0}
.end
