//! Main entrypoint to the firmware.
//! `app_main` resets the boot partition to factory and starts ember_tasking.
//! `app_main` exits and leaves behind the rate tasks to continue running.
use std::{panic, thread::sleep, time::Duration};

use ccmn_eol_shared::adc::AdcUnit;
use esp_idf_sys::{
    adc_channel_t_ADC_CHANNEL_0, adc_channel_t_ADC_CHANNEL_1, adc_unit_t_ADC_UNIT_1, esp_restart,
};

use crate::{
    canrx, canrx_is_node_ok,
    ember_tasking::{ember_rate_funcs_S, ember_tasking_begin},
};

// some extern declarations
extern "C" {
    // temp: skip generating bindings to ember-bltools for now
    fn ember_bltools_set_boot_partition_to_factory();
    static can_rf: ember_rate_funcs_S;
}

// ember_task_list and ember_task_count
#[no_mangle]
static ember_task_list: [&ember_rate_funcs_S; 2] = [unsafe { &can_rf }, &crate::leds::RATE_FUNCS];

#[no_mangle]
static ember_task_count: usize = ember_task_list.len();

// app_main
#[no_mangle]
extern "C" fn app_main() {
    panic::set_hook(Box::new(|info| {
        println!("eol tester panic! {info}");
    }));

    unsafe {
        ember_bltools_set_boot_partition_to_factory();

        println!("***~~~ CCMN EOL Testing DUT ~~~***");
        println!("firmware githash: {}", git_version::git_version!());
        println!("starting tasking...\n");

        ember_tasking_begin();

        // while !canrx_is_node_ok!(TESTER) {
        //     sleep(Duration::from_millis(20));
        //     println!("waiting for tester... {}", canrx!(TESTER_currentGpio));
        // }

        // dbg!(do_tests()).ok();

        let adc =
            AdcUnit::new_and_init(&[adc_channel_t_ADC_CHANNEL_1], adc_unit_t_ADC_UNIT_1).unwrap();

        loop {
            dbg!(adc.read(adc_channel_t_ADC_CHANNEL_1)).ok();
            sleep(Duration::from_millis(20));
        }

        esp_restart();
    }
}

fn do_tests() -> anyhow::Result<()> {
    crate::eeprom::eeprom_eol_test()?;
    crate::gpiotest::do_gpio_test()?;

    Ok(())
}
