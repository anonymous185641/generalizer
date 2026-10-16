.outputs
.dummy Epsilon recvfrom_Bank_askpay_4 recvfrom_Seller_cancel_6 recvfrom_Seller_conf_7 recvfrom_Seller_price_2 sendto_Bank_infopay_5 sendto_Seller_askprice_1 sendto_Seller_buy_3 
.state graph
q0 sendto_Seller_askprice_1 q1
q1 recvfrom_Seller_price_2 q0
q0 sendto_Seller_buy_3 q2
q2 recvfrom_Bank_askpay_4 q3
q3 sendto_Bank_infopay_5 q4
q4 recvfrom_Seller_cancel_6 q0
q4 recvfrom_Seller_conf_7 q0
q0 Epsilon qF97247
.marking {q0}
.end
