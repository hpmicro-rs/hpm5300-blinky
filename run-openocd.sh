
OPENOCD_CFG=/Users/mono/Elec/HPM/hpm_sdk/boards/openocd

openocd -f $OPENOCD_CFG/probes/ft2232.cfg -f $OPENOCD_CFG/soc/hpm5300.cfg -f $OPENOCD_CFG/boards/hpm5300evk.cfg
