#![no_std]
#![no_main]

mod clock;
mod display;
mod gui;
mod nixie;

use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals=true)]
mod app {

    use crate::{
        clock::RtcClock,
        display::Display,
        gui::Gui,
        nixie::{NibbleWriter, NixieDriver},
    };
    use panic_halt as _;
    use ssd1306::{prelude::*, size::DisplaySize128x32, I2CDisplayInterface, Ssd1306};
    use stm32f1xx_hal::{
        gpio::{
            gpioa::{PA0, PA1, PA11, PA12, PA15, PA2, PA3, PA4, PA5, PA6, PA7},
            gpiob::{PB10, PB11, PB12, PB13, PB14, PB15, PB4, PB5, PB6, PB7},
            gpioc::PC13,
            Alternate, Edge, ExtiPin, Input, OpenDrain, Output, PullUp, PushPull,
        },
        i2c::{BlockingI2c, DutyCycle, Mode},
        pac::{I2C2, TIM3},
        prelude::*,
        rtc::Rtc,
        timer::{delay::SysDelay, CounterHz, Event},
    };
    use ufmt::derive::uDebug;

    type OledScl = PB10<Alternate<OpenDrain>>;
    type OledSda = PB11<Alternate<OpenDrain>>;
    type GuiInterface = Gui<I2CInterface<BlockingI2c<I2C2, (OledScl, OledSda)>>, DisplaySize128x32>;
    type OutputPP = Output<PushPull>;

    #[derive(uDebug, Copy, Clone)]
    pub enum PressedButton {
        Left,
        Right,
        ShortPress,
        LongPress,
        None,
    }

    pub struct Buttons {
        enter: PA15<Input<PullUp>>,
        left: PA11<Input<PullUp>>,
        right: PA12<Input<PullUp>>,
    }

    #[shared]
    struct SharedResource {
        led: PC13<OutputPP>,
        tim: CounterHz<TIM3>,
        delay: SysDelay,
        buttons: Buttons,
        gui: GuiInterface,
        clock: RtcClock,
        pressed_btn: PressedButton,
        press_counter: u8,
    }

    #[local]
    struct LocalResource {
        nixie_driver: NixieDriver<
            PB12<OutputPP>,
            PB13<OutputPP>,
            PB14<OutputPP>,
            PB15<OutputPP>,
            PB4<OutputPP>,
            PB5<OutputPP>,
            PB6<OutputPP>,
            PB7<OutputPP>,
            PA4<OutputPP>,
            PA5<OutputPP>,
            PA6<OutputPP>,
            PA7<OutputPP>,
            PA0<OutputPP>,
            PA1<OutputPP>,
            PA2<OutputPP>,
            PA3<OutputPP>,
        >,
    }

    #[init]
    fn init(cx: init::Context) -> (SharedResource, LocalResource, init::Monotonics) {
        let cp = cx.core;
        let mut dp = cx.device;

        let mut flash = dp.FLASH.constrain();
        let rcc = dp.RCC.constrain();
        let mut afio = dp.AFIO.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(48.MHz())
            .pclk1(6.MHz())
            .freeze(&mut flash.acr);

        let mut gpioc = dp.GPIOC.split();
        let mut gpioa = dp.GPIOA.split();
        let mut gpiob = dp.GPIOB.split();

        /*
         * Led indicator flashing every second to show all OK
         */
        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        /*
         * Timer used to count the length of
         * time seconds button is pressed
         */
        let mut timer3 = dp.TIM3.counter_hz(&clocks);
        timer3.start(1.Hz()).unwrap();
        timer3.listen(Event::Update);

        /*
         * General purpose delay
         */
        let delay = cp.SYST.delay(&clocks);

        /*
         *  I2C config for OLED display
         *  PB10 -> SCL
         *  PB11 -> SDA
         */

        let oled_scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
        let oled_sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);
        let oled_i2c = BlockingI2c::i2c2(
            dp.I2C2,
            (oled_scl, oled_sda),
            Mode::Fast {
                frequency: 400_000.Hz(),
                duty_cycle: DutyCycle::Ratio2to1,
            },
            clocks,
            1000,
            10,
            1000,
            1000,
        );
        let interface = I2CDisplayInterface::new(oled_i2c);
        let mut oled = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate180)
            .into_buffered_graphics_mode();
        oled.init().unwrap();
        let display = Display::new(oled);
        let gui = Gui::new(display);

        /*
         * Bunch of buttons for setting up of the clock time
         */
        let (pa15, _, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
        let mut enter = pa15.into_pull_up_input(&mut gpioa.crh);
        enter.make_interrupt_source(&mut afio);
        enter.trigger_on_edge(&dp.EXTI, Edge::RisingFalling);
        enter.enable_interrupt(&dp.EXTI);
        let mut left = gpioa.pa11.into_pull_up_input(&mut gpioa.crh); // PA11
        left.make_interrupt_source(&mut afio);
        left.trigger_on_edge(&dp.EXTI, Edge::Falling);
        left.enable_interrupt(&dp.EXTI);
        let mut right = gpioa.pa12.into_pull_up_input(&mut gpioa.crh); //PA12
        right.make_interrupt_source(&mut afio);
        right.trigger_on_edge(&dp.EXTI, Edge::Falling);
        right.enable_interrupt(&dp.EXTI);

        let buttons = Buttons { enter, left, right };

        // Initialize RTC
        let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut dp.PWR);
        let rtc = Rtc::new(dp.RTC, &mut backup_domain);
        let clock = RtcClock::new(rtc);

        /*
         *  Configure pins used for driving the Nixie tubes
         */
        let min_u_d0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
        let min_u_d1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
        let min_u_d2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
        let min_u_d3 = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
        let min_u = NibbleWriter::new(min_u_d0, min_u_d1, min_u_d2, min_u_d3);

        let min_t_d0 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
        let min_t_d1 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
        let min_t_d2 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
        let min_t_d3 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
        let min_t = NibbleWriter::new(min_t_d0, min_t_d1, min_t_d2, min_t_d3);

        let hour_u_d0 = pb4.into_push_pull_output(&mut gpiob.crl);
        let hour_u_d1 = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
        let hour_u_d2 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
        let hour_u_d3 = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
        let hour_u = NibbleWriter::new(hour_u_d0, hour_u_d1, hour_u_d2, hour_u_d3);

        let hour_t_d0 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
        let hour_t_d1 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
        let hour_t_d2 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
        let hour_t_d3 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);
        let hour_t = NibbleWriter::new(hour_t_d0, hour_t_d1, hour_t_d2, hour_t_d3);

        let nixie_driver = NixieDriver::new(hour_t, hour_u, min_t, min_u);
        (
            SharedResource {
                led,
                tim: timer3,
                delay,
                gui,
                buttons,
                clock,
                press_counter: 0,
                pressed_btn: PressedButton::None,
            },
            LocalResource { nixie_driver },
            init::Monotonics(),
        )
    }

    #[idle(shared = [delay, gui, clock, pressed_btn], local=[nixie_driver])]
    fn idle(cx: idle::Context) -> ! {
        let delay = cx.shared.delay;
        let gui = cx.shared.gui;
        let clock = cx.shared.clock;
        let mut pressed_btn = cx.shared.pressed_btn;
        let nixie_driver = cx.local.nixie_driver;
        (clock, gui, delay).lock(|clock, gui, delay| {
            // draw stuff here
            loop {
                // Update buttons
                let pb = pressed_btn.lock(|pb| *pb);
                match pb {
                    PressedButton::Left => {
                        gui.backward();
                        pressed_btn.lock(|pb| *pb = PressedButton::None);
                    }
                    PressedButton::Right => {
                        gui.forward();
                        pressed_btn.lock(|pb| *pb = PressedButton::None);
                    }
                    PressedButton::LongPress => {
                        gui.edit_clock(clock);
                        pressed_btn.lock(|pb| *pb = PressedButton::None);
                    }
                    PressedButton::ShortPress => {
                        gui.select();
                        pressed_btn.lock(|pb| *pb = PressedButton::None);
                    }
                    _ => {}
                };

                gui.print_clock(clock);
                let time = clock.get_time();
                nixie_driver.show(&time);

                delay.delay_ms(500u32);
            }
        })
    }

    #[task(binds = EXTI15_10, shared = [buttons, press_counter, pressed_btn])]
    fn exti15_10(cx: exti15_10::Context) {
        let buttons = cx.shared.buttons;
        let press_counter = cx.shared.press_counter;
        let pressed_btn = cx.shared.pressed_btn;

        (buttons, press_counter, pressed_btn).lock(|buttons, pc, pb| {
            let Buttons { enter, left, right } = buttons;
            if enter.check_interrupt() {
                if enter.is_low() {
                    /* Reset counter on buton press */
                    *pc = 0;
                } else if *pc > 3 && enter.is_high() {
                    /* If the button was held for 3 seconds its a Long press */
                    *pb = PressedButton::LongPress;
                } else {
                    *pb = PressedButton::ShortPress;
                }
            } else if left.check_interrupt() && left.is_low() {
                *pb = PressedButton::Left;
            } else if right.check_interrupt() && right.is_low() {
                *pb = PressedButton::Right;
            }

            enter.clear_interrupt_pending_bit();
            left.clear_interrupt_pending_bit();
            right.clear_interrupt_pending_bit();
        })
    }

    #[task(binds = TIM3, shared = [led, tim, press_counter])]
    fn tim3(mut cx: tim3::Context) {
        let _ = cx.shared.led.lock(|led| led.toggle());
        let _ = cx.shared.tim.lock(|tim| tim.wait());
        cx.shared.press_counter.lock(|pc| *pc += 1);
    }
}
