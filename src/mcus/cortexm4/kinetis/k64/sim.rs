extern crate core;
use core::intrinsics::{volatile_load, volatile_store};


ioreg!(
    name => SIM;
    doc_srcs => [
        "http://www.nxp.com/files/microcontrollers/doc/ref_manual/K64P144M120SF5RM.pdf" // section 12
    ];

    constants => {
        enabled         = 1;
        disabled        = 0;
    };


    //
    // usb voltage
    //

    0x0000 => options_1 r32 rw {
        // ... reserved ...

        12..15 => {
            // TODO: read only part of a register
            //       this register contains information about RAM size, but we rely on the linker script
        }

        // ... reserved ...

        18..19 => {
            use_system_oscillator => [0x0];
            use_rtc_oscillator => [0x2];
            use_lpo_oscillator => [0x3];
        }

        // ... reserved ...

        29 => {
            set_usb_voltage_regulator_low_power_mode => [enabled];
            unset_usb_voltage_regulator_low_power_mode => [disabled];
        }

        30 => {
            set_usb_voltage_regulator_standby_mode => [enabled];
            unset_usb_voltage_regulator_standby_mode => [disabled];
        }

        31 => {
            set_usb_voltage_regulator_enable => [enabled];
            unset_usb_voltage_regulator_enable => [disabled];
        }
    };

    0x0004 => options_1_config r32 rw {
        // ... reserved ...

        24 => {
            enable_usb_voltage_regulator_write => [enabled];
            disable_usb_voltage_regulator_write => [disabled];
        }

        25 => {
            enable_usb_voltage_regulator_low_power_mode_write => [enabled];
            disable_usb_voltage_regulator_low_power_mode_write => [disabled];
        }

        26 => {
            enable_usb_voltage_regulator_standby_mode_write => [enabled];
            disable_usb_voltage_regulator_standby_mode_write => [disabled];
        }

        // ... reserved ...
    };


    //
    // clocking
    //

    // NOTE: large gap in register addresses
    0x1004 => options_2 r32 rw {
        // ... reserved ...

        4 => {
            rtc_clkout_use_1hz          => [0];
            rtc_clkout_use_32hz         => [1];
        }

        5..7 => {
            clkout_use_flexbus_clock    => [0b000];
            clkout_use_flash_clock      => [0b010];
            clkout_use_lpo_clock        => [0b011]; // 1Hz clock
            clkout_use_mcgir_clock      => [0b100];
            clkout_use_rtc_clock        => [0b101]; // 32.xyz Hz clock
            clkout_use_oscer_clock      => [0b110];
            clkout_use_irc_clock        => [0b111]; // 48 MHz clock

        }

        8..9 => {
            disallow_flexbus_access     => [0]; // 0b01 is also valid, but why bother?
            allow_flexbus_data_access   => [2];
            allow_flexbus_all_access    => [3];
        }

        // ... reserved ...

        11 => {
            use_single_pad_ptd7_drive_strength => [0];
            use_double_pad_ptd7_drive_strength => [1];
        }

        12 => {
            use_mcg_clock_for_debug     => [0];
            use_system_clock_for_debug  => [1];
        }

        // ... reserved ...

        16..17 => {
            use_mcgfll_peripheral_clock => [0b00];
            use_mcgpll_peripheral_clock => [0b01];
            use_irc_peripheral_clock    => [0b11]; // 48 MHz clock
        }

        18 => {
            usb_use_external_clock      => [0];
            usb_use_internal_clock      => [1];
        }

        19 => {
            rmii_use_extal_clock        => [0];
            rmii_use_external_clock     => [1];
        }

        20..21 => {
            ethernet_use_system_clock   => [0b00];
            ehternet_use_pllfll_clock   => [0b01];
            ethernet_use_oscer_clock    => [0b10];
            ethernet_use_external_clock => [0b11];
        }

        // ... reserved ...

        28..29 => {
            sdhc_use_system_clock       => [0b00];
            sdhc_use_pllfll_clock       => [0b01];
            sdhc_use_oscer_clock        => [0b10];
            sdhc_use_external_clock     => [0b11];
        }

        // ... reserved ...
    };


    // NOTE: no sim options 3 register....?


    //
    // flex timer
    //

    0x100C => options_4 r32 rw {
        0 => {
            flextimer_0_fault_0_use_pin_source  => [0];
            flextimer_0_fault_0_use_cmp_source  => [1];
        }

        1 => {
            flextimer_0_fault_1_use_pin_source  => [0];
            flextimer_0_fault_1_use_cmp_source  => [1];
        }

        2 => {
            flextimer_0_fault_2_use_pin_source  => [0];
            flextimer_0_fault_2_use_cmp_source  => [1];
        }

        // ... reserved ...

        4 => {
            flextimer_1_fault_0_use_pin_source  => [0];
            flextimer_1_fault_0_use_cmp_source  => [1];
        }

        // ... reserved ...

        8 => {
            flextimer_2_fault_0_use_pin_source  => [0];
            flextimer_2_fault_0_use_cmp_source  => [1];
        }

        // ... reserved ...

        12 => {
            flextimer_3_fault_0_use_pin_source  => [0];
            flextimer_3_fault_0_use_cmp_source  => [1];
        }

        // ... reserved ...

        18..19 => {
            flextimer_1_use_internal_channel    => [0];
            flextimer_1_use_cmp0_channel        => [1];
            flextimer_1_use_cmp1_channel        => [2];
            flextimer_1_use_usb_frame_start_channel => [3];
        }

        20..21 => {
            flextimer_2_use_internal_channel    => [0];
            flextimer_2_use_cmp0_channel        => [1];
            flextimer_2_use_cmp1_channel        => [2];
        }

        // ... reserved ...

        24 => {
            flextimer_0_use_clk0_pin_clock      => [0];
            flextimer_0_use_clk1_pin_clock      => [1];
        }

        25 => {
            flextimer_1_use_clk0_pin_clock      => [0];
            flextimer_1_use_clk1_pin_clock      => [1];
        }

        26 => {
            flextimer_2_use_clk0_pin_clock      => [0];
            flextimer_2_use_clk1_pin_clock      => [1];
        }

        27 => {
            flextimer_3_use_clk0_pin_clock      => [0];
            flextimer_3_use_clk1_pin_clock      => [1];
        }

        28 => {
            flextimer_0_hw_trigger0_use_hscmp   => [0];
            flextimer_0_hw_trigger0_use_ftm1    => [1];
        }

        29 => {
            flextimer_0_hw_trigger1_use_pdb     => [0];
            flextimer_0_hw_trigger1_use_ftm2    => [1];
        }

        30 => {
            flextimer_3_hw_trigger0_use_ftm1    => [1];
        }

        31 => {
            flextimer_3_hw_trigger1_use_ftm2    => [1];
        }
    };



    //
    // UART{0, 1} TX/RX sourcing
    //

    0x1010 => options_5 r32 rw {
        //
        // uart 0
        //

        0..1 => {
            uart_0_tx_use_raw_pin               => [0];
            uart_0_tx_use_modulated_ftm1        => [1];
            uart_0_tx_use_modulated_ftm2        => [2];
        }

        2..3 => {
            uart_0_rx_use_rx_pin                => [0];
            uart_0_rx_use_cmp0                  => [1];
            uart_0_rx_use_cmp1                  => [2];
        }

        //
        // uart 1
        //

        4..5 => {
            uart_1_tx_use_raw_pin               => [0];
            uart_1_tx_use_modulated_ftm1        => [1];
            uart_1_tx_use_modulated_ftm2        => [2];
        }

        6..7 => {
            uart_1_rx_use_rx_pin                => [0];
            uart_1_rx_use_cmp0                  => [1];
            uart_1_rx_use_cmp1                  => [2];
        }

        // ... reserved ...
    };


    // NOTE: no sim options 6 register....?


    //
    // ADC {0, 1} partial config
    //

    0x1018 => options_7 r32 rw {
        //
        // ADC 0
        //

        0..3 => {
            adc_0_use_pdb_external_trigger      => [0b0000];
            adc_0_use_hscmp_0_trigger           => [0b0001];
            adc_0_use_hscmp_1_trigger           => [0b0010];
            adc_0_use_hscmp_2_trigger           => [0b0011];
            adc_0_use_pit_0_trigger             => [0b0100];
            adc_0_use_pit_1_trigger             => [0b0101];
            adc_0_use_pit_2_trigger             => [0b0110];
            adc_0_use_pit_3_trigger             => [0b0111];
            adc_0_use_ftm_0_trigger             => [0b1000];
            adc_0_use_ftm_1_trigger             => [0b1001];
            adc_0_use_ftm_2_trigger             => [0b1010];
            adc_0_use_ftm_3_trigger             => [0b1011];
            adc_0_use_rtc_alarm_trigger         => [0b1100];
            adc_0_use_rtc_seconds_trigger       => [0b1101];
            adc_0_use_low_power_trigger         => [0b1110];
        }

        4 => {
            adc_0_use_pretrigger_a              => [0];
            adc_0_use_pretrigger_b              => [1];
        }

        // ... reserved ...

        7 => {
            adc_0_use_alt_trigger_pdb           => [0];
            adc_0_use_alt_trigger               => [1];
        }

        //
        // ADC 1
        //

        8..11 => {
            adc_1_use_pdb_external_trigger      => [0b0000];
            adc_1_use_hscmp_0_trigger           => [0b0001];
            adc_1_use_hscmp_1_trigger           => [0b0010];
            adc_1_use_hscmp_2_trigger           => [0b0011];
            adc_1_use_pit_0_trigger             => [0b0100];
            adc_1_use_pit_1_trigger             => [0b0101];
            adc_1_use_pit_2_trigger             => [0b0110];
            adc_1_use_pit_3_trigger             => [0b0111];
            adc_1_use_ftm_0_trigger             => [0b1000];
            adc_1_use_ftm_1_trigger             => [0b1001];
            adc_1_use_ftm_2_trigger             => [0b1010];
            adc_1_use_ftm_3_trigger             => [0b1011];
            adc_1_use_rtc_alarm_trigger         => [0b1100];
            adc_1_use_rtc_seconds_trigger       => [0b1101];
            adc_1_use_low_power_trigger         => [0b1110];
        }

        12 => {
            adc_1_use_pretrigger_a              => [0];
            adc_1_use_pretrigger_b              => [1];
        }

        // ... reserved ...

        15 => {
            adc_1_use_alt_trigger_pdb           => [0];
            adc_1_use_alt_trigger               => [1];
        }

        // ... reserved ...
    };


    //
    // system identification
    //

    0x1024 => system_id r32 ro { /* TODO: read portions of the register to get this info */ };


    //
    // clock gating
    //

    0x1028 => clock_gating_1 r32 rw {
        // ... reserved ...

        6 => {
            i2c_2_disable_clock                 => [0];
            i2c_2_enable_clock                  => [1];
        }

        // ... reserved ...

        10 => {
            uart_4_disable_clock                => [0];
            uart_4_enable_clock                 => [1];
        }

        11 => {
            uart_5_disable_clock                => [0];
            uart_5_enable_clock                 => [1];
        }

        // ... reserved ...
    };

    0x102C => clock_gating_2 r32 rw {
        0 => {
            ethernet_disable_clock              => [0];
            ethernet_enable_clock               => [1];
        }

        // ... reserved ...

        12 => {
            dac_0_disable_clock                 => [0];
            dac_0_enable_clock                  => [1];
        }

        13 => {
            dac_1_disable_clock                 => [0];
            dac_1_enable_clock                  => [1];
        }
    };

    0x1030 => clock_gating_3 r32 rw {
        0 => {
            rnga_disable_clock                  => [0];
            rnga_enable_clock                   => [1];
        }

        // ... reserved ...

        12 => {
            spi_2_disable_clock                 => [0];
            spi_2_enable_clock                  => [1];
        }

        // ... reserved ...

        17 => {
            sdhc_disable_clock                  => [0];
            sdhc_enable_clock                   => [1];
        }

        // ... reserved ...

        24 => {
            ftm_2_disable_clock                 => [0];
            ftm_2_enable_clock                  => [1];
        }

        25 => {
            ftm_3_disable_clock                 => [0];
            ftm_3_enable_clock                  => [1];
        }

        // ... reserved ...

        27 => {
            adc_1_disable_clock                 => [0];
            adc_1_enable_clock                  => [1];
        }

        // ... reserved ...
    };

    0x1034 => clock_gating_4 r32 rw {
        // ... reserved ...

        1 => {
            ewm_disable_clock                   => [0];
            ewm_enable_clock                    => [1];
        }

        2 => {
            cmt_disable_clock                   => [0];
            cmt_enable_clock                    => [1];
        }

        // ... reserved ...

        6 => {
            i2c_0_disable_clock                 => [0];
            i2c_0_enable_clock                  => [1];
        }

        7 => {
            i2c_1_disable_clock                 => [0];
            i2c_1_enable_clock                  => [1];
        }

        // ... reserved ...

        10 => {
            uart_0_disable_clock                => [0];
            uart_0_enable_clock                 => [1];
        }

        11 => {
            uart_1_disable_clock                => [0];
            uart_1_enable_clock                 => [1];
        }

        12 => {
            uart_2_disable_clock                => [0];
            uart_2_enable_clock                 => [1];
        }

        13 => {
            uart_3_disable_clock                => [0];
            uart_3_enable_clock                 => [1];
        }

        // ... reserved ...

        18 => {
            usb_disable_clock                   => [0];
            usb_enable_clock                    => [1];
        }

        19 => {
            cmp_disable_clock                   => [0];
            cmp_enable_clock                    => [1];
        }

        20 => {
            vref_disable_clock                  => [0];
            vref_enable_clock                   => [1];
        }

        // ... reserved ...
    };

    0x1038 => clock_gating_5 r32 rw {
        0 => {
            disable_low_power_timer_access      => [0];
            enable_low_power_timer_access       => [1];
        }

        // ... reserved ...

        9 => {
            port_a_disable_clock                => [0];
            port_a_enable_clock                 => [1];
        }

        10 => {
            port_b_disable_clock                => [0];
            port_b_enable_clock                 => [1];
        }

        11 => {
            port_c_disable_clock                => [0];
            port_c_enable_clock                 => [1];
        }

        12 => {
            port_d_disable_clock                => [0];
            port_d_enable_clock                 => [1];
        }

        13 => {
            port_e_disable_clock                => [0];
            port_e_enable_clock                 => [1];
        }

        // ... reserved ...
    };

    0x103C => clock_gating_6 r32 rw {
        0 => {
            flash_disable_clock                 => [0];
            flash_enable_clock                  => [1];
        }

        1 => {
            dma_mux_disable_clock               => [0];
            dma_mux_enable_clock                => [1];
        }

        // ... reserved ...

        4 => {
            can_0_disable_clock                 => [0];
            can_0_enable_clock                  => [1];
        }

        // ... reserved ...

        // TODO: bit 9 claims for RNGA, but we already have one of those.... and the table block is emtpy?
        //       error in the data sheet?

        // ... reserved ...

        12 => {
            spi_0_disable_clock                 => [0];
            spi_0_enable_clock                  => [1];
        }

        13 => {
            spi_1_disable_clock                 => [0];
            spi_1_enable_clock                  => [1];
        }

        // ... reserved ...

        15 => {
            i2s_disable_clock                   => [0];
            i2s_enable_clock                    => [1];
        }

        // ... reserved ...

        18 => {
            crc_disable_clock                   => [0];
            crc_enable_clock                    => [1];
        }

        // ... reserved ...

        21 => {
            usb_dcd_disable_clock               => [0];
            usb_dcd_enable_clock                => [1];
        }

        22 => {
            pdb_disable_clock                   => [0];
            pdb_enable_clock                    => [1];
        }

        23 => {
            pit_disable_clock                   => [0];
            pit_enable_clock                    => [1];
        }

        24 => {
            ftm_0_disable_clock                 => [0];
            ftm_0_enable_clock                  => [1];
        }

        25 => {
            ftm_1_disable_clock                 => [0];
            ftm_1_enable_clock                  => [1];
        }

        /*  TODO: yet another duplicated clock bit -- needs both? datasheet errata?
        26 => {
            ftm_2_disable_clock                 => [0];
            ftm_2_enable_clock                  => [1];
        }
        */

        27 => {
            adc_0_disable_clock                 => [0];
            adc_0_enable_clock                  => [1];
        }

        // ... reserved ...

        29 => {
            rtc_disable_access                  => [0];
            rtc_enable_access                   => [1];
        }

        // ... reserved ...

        /*  TODO: another duplicate clock gate... already in clock_gating_2. needs both!?
        31 => {
            dac_0_disable_clock                 => [0];
            dac_0_enable_clock                  => [1];
        }
        */
    };

    0x1040 => clock_gating_7 r32 rw {
        0 => {
            flexbus_disable_clock               => [0];
            flexbus_enable_clock                => [0];
        }

        1 => {
            dma_disable_clock                   => [0];
            dma_enable_clock                    => [0];
        }

        2 => {
            mpu_disable_clock                   => [0];
            mpu_enable_clock                    => [0];
        }

        // ... reserved ...
    };


    //
    // clock division
    //

    0x1040 => clock_divider_1 r32 rw {
        // ... reserved ...

        16..19 => { // OUTDIV4
            set_flash_clock_divide_1            => [0b0000];
            set_flash_clock_divide_2            => [0b0001];
            set_flash_clock_divide_3            => [0b0010];
            set_flash_clock_divide_4            => [0b0011];
            set_flash_clock_divide_5            => [0b0100];
            set_flash_clock_divide_6            => [0b0101];
            set_flash_clock_divide_7            => [0b0110];
            set_flash_clock_divide_8            => [0b0111];
            set_flash_clock_divide_9            => [0b1000];
            set_flash_clock_divide_10           => [0b1001];
            set_flash_clock_divide_11           => [0b1010];
            set_flash_clock_divide_12           => [0b1011];
            set_flash_clock_divide_13           => [0b1100];
            set_flash_clock_divide_14           => [0b1101];
            set_flash_clock_divide_15           => [0b1110];
            set_flash_clock_divide_16           => [0b1111];
        }

        20..23 => { // OUTDIV3
            set_flexbus_clock_divide_1          => [0b0000];
            set_flexbus_clock_divide_2          => [0b0001];
            set_flexbus_clock_divide_3          => [0b0010];
            set_flexbus_clock_divide_4          => [0b0011];
            set_flexbus_clock_divide_5          => [0b0100];
            set_flexbus_clock_divide_6          => [0b0101];
            set_flexbus_clock_divide_7          => [0b0110];
            set_flexbus_clock_divide_8          => [0b0111];
            set_flexbus_clock_divide_9          => [0b1000];
            set_flexbus_clock_divide_10         => [0b1001];
            set_flexbus_clock_divide_11         => [0b1010];
            set_flexbus_clock_divide_12         => [0b1011];
            set_flexbus_clock_divide_13         => [0b1100];
            set_flexbus_clock_divide_14         => [0b1101];
            set_flexbus_clock_divide_15         => [0b1110];
            set_flexbus_clock_divide_16         => [0b1111];
        }

        24..27 => { // OUTDIV2
            set_bus_clock_divide_1              => [0b0000];
            set_bus_clock_divide_2              => [0b0001];
            set_bus_clock_divide_3              => [0b0010];
            set_bus_clock_divide_4              => [0b0011];
            set_bus_clock_divide_5              => [0b0100];
            set_bus_clock_divide_6              => [0b0101];
            set_bus_clock_divide_7              => [0b0110];
            set_bus_clock_divide_8              => [0b0111];
            set_bus_clock_divide_9              => [0b1000];
            set_bus_clock_divide_10             => [0b1001];
            set_bus_clock_divide_11             => [0b1010];
            set_bus_clock_divide_12             => [0b1011];
            set_bus_clock_divide_13             => [0b1100];
            set_bus_clock_divide_14             => [0b1101];
            set_bus_clock_divide_15             => [0b1110];
            set_bus_clock_divide_16             => [0b1111];
        }

        28..31 => { // OUTDIV1
            set_system_clock_divide_1           => [0b0000];
            set_system_clock_divide_2           => [0b0001];
            set_system_clock_divide_3           => [0b0010];
            set_system_clock_divide_4           => [0b0011];
            set_system_clock_divide_5           => [0b0100];
            set_system_clock_divide_6           => [0b0101];
            set_system_clock_divide_7           => [0b0110];
            set_system_clock_divide_8           => [0b0111];
            set_system_clock_divide_9           => [0b1000];
            set_system_clock_divide_10          => [0b1001];
            set_system_clock_divide_11          => [0b1010];
            set_system_clock_divide_12          => [0b1011];
            set_system_clock_divide_13          => [0b1100];
            set_system_clock_divide_14          => [0b1101];
            set_system_clock_divide_15          => [0b1110];
            set_system_clock_divide_16          => [0b1111];
        }
    };

    0x1048 => clock_divider_2 r32 rw {
        0 => {    set_usb_clock_fraction        => (); }
        1..3 => { set_usb_clock_divisor         => (); }

        // ... reserved ...
    };


    //
    // flash configuration
    //

    0x104C => flash_configuration_1 r32 ro {};
    0x1050 => flash_configuration_2 r32 ro {};

    //
    // unique system id
    //

    0x1054 => unique_id_high r32 ro {};
    0x1058 => unique_id_mid_high r32 ro {};
    0x105C => unique_id_mid_low r32 ro {};
    0x1060 => unique_id_low r32 ro {};
);
