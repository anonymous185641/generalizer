.outputs
.dummy Epsilon recvfrom_A0_cwin_2 recvfrom_A1_close_6 sendto_A0_msg_4 sendto_A1_blose_7 sendto_A3_busy_8 
.state graph
carol0 recvfrom_A1_close_6 carol3
carol0 sendto_A3_busy_8 carol2
carol0 recvfrom_A0_cwin_2 carol1
carol1 sendto_A3_busy_8 carol4
carol1 sendto_A1_blose_7 carol3
carol2 recvfrom_A1_close_6 carol5
carol2 recvfrom_A0_cwin_2 carol4
carol3 sendto_A3_busy_8 carol5
carol4 sendto_A1_blose_7 carol5
carol5 sendto_A0_msg_4 carol0
carol0 Epsilon qF12078
.marking {carol0}
.end
