# wiobldc

Driving BLDC for customized wio terminal clone board.

## Write to ATSAMD51P20A with openocd or uf2
You must build customised uf2 bootloader


```
/app/boards# cat yutaka_p20a/board_config.h
#ifndef BOARD_CONFIG_H
#define BOARD_CONFIG_H

#define VENDOR_NAME "Yutaka"
#define PRODUCT_NAME "YutakaCustom"
#define VOLUME_LABEL "Yutaka"

#define BOARD_ID "ATSAMD51P20A-v0"

#define USB_VID 0x2886
#define USB_PID 0x002d

#define LED_PIN PIN_PA15

#define BOOT_GCLK_ID_CORE                 SERCOM0_GCLK_ID_CORE
#define BOOT_GCLK_ID_SLOW                 SERCOM0_GCLK_ID_SLOW
#define BOOT_USART_MASK                   APBAMASK
#define BOOT_USART_BUS_CLOCK_INDEX        MCLK_APBAMASK_SERCOM0

#endif


/app/boards# cat yutaka_p20a/board.mk
CHIP_FAMILY = samd51
CHIP_VARIANT = SAMD51P20A
```

ビルド
```
make BOARD=yutaka_p20a
```

### example with openocd
```
# flush
openocd -d2 -f ./cmsis-dap.cfg -f ./atsame5x_custom.cfg -c "set CHIPNAME samd51g20a; adapter_nsrst_delay 100; adapter_nsrst_assert_width 100; init; targets; reset halt; atsame5 chip-erase;"


# uf2 バイナリ 書き込み 0x0000から
openocd -d2 -f ./cmsis-dap.cfg -f ./atsame5x_custom.cfg -c "set CHIPNAME samd51g20a; adapter_nsrst_delay 100; adapter_nsrst_assert_width 100; init; targets; reset halt; atsame5 bootloader 0; flash write_image build/yutaka_p20a/bootloader-yutaka_p20a-v3.14.0-10-gb17ca18-dirty.bin 0x0000 ; reset halt; flash verify_image build/yutaka_p20a/bootloader-yutaka_p20a-v3.14.0-10-gb17ca18-dirty.bin 0x0000;"

# rename 
objcopy -O binary wiobldc wiobldc.bin

# write
openocd -d2 -f ./cmsis-dap.cfg -f ./atsame5x_custom.cfg -c "set CHIPNAME samd51g20a; adapter_nsrst_delay 100; adapter_nsrst_assert_width 100; init; targets; reset halt; atsame5 bootloader 0; flash write_image ./wiobldc.bin 0x4000 ; reset halt; flash verify_image ./wiobldc.bin 0x4000;"

# reset 
openocd -d2 -f ./cmsis-dap.cfg -f ./atsame5x_custom.cfg -c "init; reset;"
```

