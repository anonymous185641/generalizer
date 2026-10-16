.outputs
.dummy Epsilon recvfrom_A1_sig_3 recvfrom_A2_msg_4 sendto_A1_bwin_1 sendto_A2_cwin_2 sendto_A3_free_5 
.state graph
alice0 sendto_A1_bwin_1 alice1
alice0 sendto_A2_cwin_2 alice1
alice1 recvfrom_A1_sig_3 alice2
alice1 recvfrom_A2_msg_4 alice3
alice2 recvfrom_A2_msg_4 alice4
alice3 recvfrom_A1_sig_3 alice4
alice4 sendto_A3_free_5 alice0
alice0 Epsilon qF12040
.marking {alice0}
.end
