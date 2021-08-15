use crate::{generation::LOG_I32_FUNC_NAME, wat};
use super::{Wat, ToWat, ToWatVec};

pub struct StdLib;

const I32_LOG_4 : &'static str = "i32_log_4";
const I32_POW_4 : &'static str = "i32_pow_4";

impl StdLib {
    pub fn new() -> Self {
        Self
    }

    pub fn get_header(&self) -> Vec<Wat> {
        vec![
            Wat::import_function("log", "i32", LOG_I32_FUNC_NAME, vec!["i32"], None),
            Wat::import_function("log", "special", "log_special", vec![], None),
        ]
    }

    pub fn get_functions(&self) -> Vec<Wat> {
        vec![
            self.get_log_4_func(),
            self.get_pow_4_func()
        ]
    }

    fn get_log_4_func(&self) -> Wat {
        Wat::declare_function(I32_LOG_4, None, vec![("n", "i32")], Some("i32"), vec![], vec![
            Wat::declare_local_i32("result"),
            Wat::if_else(
                wat!["i32.lt_s", Wat::get_local("n"), Wat::const_i32(4)],
                vec![
                    Wat::set_local("result", Wat::const_i32(1))
                ],
                vec![
                    Wat::set_local("result", Wat::const_i32(0)),
                    Wat::increment_local_i32("n", -1),
                    Wat::while_loop(
                        wat!["i32.ne", Wat::get_local("n"), Wat::const_i32(0)],
                        vec![
                            Wat::set_local("n", wat!["i32.shr_u", Wat::get_local("n"), Wat::const_i32(2)]),
                            Wat::increment_local_i32("result", 1)
                        ]
                    )
                ]
            ),
            Wat::get_local("result")
        ])
    }

    fn get_pow_4_func(&self) -> Wat {
        Wat::declare_function(I32_POW_4, None, vec![("n", "i32")], Some("i32"), vec![], vec![
            Wat::declare_local_i32("result"),

            Wat::set_local("result", Wat::const_i32(1)), // let result = 1
            Wat::while_loop(
                wat!["i32.gt_u", Wat::get_local("n"), Wat::const_i32(0)], // while (n > 0)
                vec![
                    Wat::set_local("result", wat!["i32.shl", Wat::get_local("result"), Wat::const_i32(2)]), // result = result << 2
                    Wat::increment_local_i32("n", -1) // n -= 1
                ]
            ),
            Wat::get_local("result"), // return result
        ])
    }

    pub fn log_4(&self, n: Wat) -> Wat {
        Wat::call(I32_LOG_4, vec![n])
    }

    pub fn pow_4(&self, n: Wat) -> Wat {
        Wat::call(I32_POW_4, vec![n])
    }

    pub fn log_i32(&self) -> Wat {
        Wat::call_from_stack(LOG_I32_FUNC_NAME)
    }

    pub fn array_length(&self) -> Wat {
        Wat::call_from_stack("array_length")
    }

    pub fn array_get(&self) -> Wat {
        Wat::call_from_stack("array_get")
    }
}