#[macro_export]
macro_rules! lite_main {
    () => {
        #[unsafe(no_mangle)]
        extern "C" fn mainCRTStartup() -> ! {
            main();
            $crate::exit(0)
        }
    };
}

#[macro_export]
macro_rules! lite_panic_handler {
    () => {
        #[panic_handler]
        fn panic(_: &core::panic::PanicInfo) -> ! {
            $crate::exit(1)
        }
    };

    ($handler:path $(,)?) => {
        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            $handler(info)
        }
    };
}
