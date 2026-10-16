.outputs
.dummy Epsilon recvfrom_srv_cfg_2 recvfrom_srv_png_5 sendto_usr_hbt_6 sendto_usr_sta_3 
.state graph
q0 recvfrom_srv_cfg_2 q1
q0 sendto_usr_sta_3 q0
q0 recvfrom_srv_png_5 q2
q2 sendto_usr_hbt_6 q0
q2 recvfrom_srv_cfg_2 q3
q1 sendto_usr_sta_3 q1
q1 recvfrom_srv_png_5 q3
q3 sendto_usr_hbt_6 q1
q1 Epsilon qF106725
.marking {q0}
.end
