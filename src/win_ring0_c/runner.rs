use crate::win_ring0_c::WinRing0;
use std::panic;

pub struct SimpleWinDriverRunner {
    pub initialized: bool,
    win0_driver: Option<Box<WinRing0>>
}

impl SimpleWinDriverRunner {
    pub fn new() -> SimpleWinDriverRunner {
        SimpleWinDriverRunner { initialized: false, win0_driver: None }
    }

    pub fn initialize_driver(&mut self) -> bool {
        let result = panic::catch_unwind(|| {
            let mut r0: Box<WinRing0> = Box::from(WinRing0::new());
            let _ = r0.install();
            let result_open = r0.open();

            match result_open {
                Ok(_) => (),
                Err(err) => panic!("{e}", e= err),
            }

            return r0;
        });

        match result {
            Ok(_) => {
                self.initialized = true;
                self.win0_driver = Some(result.unwrap());
                return true;
            },
            Err(_) => {
                print!("Erro ao inicializar Driver.");
                self.initialized = false;
                return false;
            },
        }
    }

    pub fn read_msr(&self, msr: u32) -> u64 {
        if self.initialized == false {
            return 0;
        }
        return self.win0_driver.as_ref().unwrap().readMsr(msr).unwrap();
    }

    pub fn read_smu(&self, msr: u32) -> u64 {
        if self.initialized == false {
            return 0;
        }

        let reference = self.win0_driver.as_ref().unwrap();
        let _ = reference.writePciConfig(msr);
        return reference.readPciConfig().unwrap();
    }

    pub fn clean_up(&mut self) -> bool {
        let reference = self.win0_driver.as_mut().unwrap();
        let _ = reference.close();
        let _ = reference.uninstall();

        self.initialized = false;
        return true;
    }
}
