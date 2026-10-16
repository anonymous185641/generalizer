.outputs
.dummy Epsilon recvfrom_A0_bwin_1 recvfrom_A2_blose_7 sendto_A0_sig_3 sendto_A2_close_6 
.state graph
bob0 recvfrom_A0_bwin_1 bob1
bob1 sendto_A2_close_6 bob2
bob0 recvfrom_A2_blose_7 bob2
bob2 sendto_A0_sig_3 bob0
bob0 Epsilon qF12062
.marking {bob0}
.end
