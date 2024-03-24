mod bin_file;
mod mem;
mod perips;
mod config;
mod rv32_actor;
mod utils;
mod intrrupt;
mod gdbserver;

// static mut NO_ACK_MODE: bool = false;



// fn pack_rsp(s: &str) -> String {
//     let mut res = String::new();

//     if unsafe { NO_ACK_MODE } {
//         res.push_str("$");
//     } else {
//         res.push_str("+$");
//     }
//     res.push_str(s);
//     res.push('#');
//     let sum = str_add_sum(s);
//     res.push_str(&u82hex(sum));

//     res
// }

// pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     let byte_content = fs::read(path)?;
//     Ok(byte_content)
// }

// struct Emu {
//     pc: u32,
//     reg: [u32; 32],
//     mem: Vec<u8>,
// }

// impl Emu {
//     fn new() -> Self {
//         match read_file("exam.bin") {
//             Ok(bytes) => Emu{ pc: 0, reg: [0; 32], mem: bytes},
//             Err(_) => {
//                 println!("file read failed.");
//                 Emu{ pc: 0, reg: [0; 32], mem: vec![0; 8192] }
//             },
//         }
//     }

//     fn s_rsp(&mut self) -> String {
//         self.pc += 4;
//         "S05".to_string()
//     }

//     fn g_rsp(&self) -> String {
//         let mut res = String::new();
//         for r in self.reg {
//             res.push_str(&u322hex(r));
//         }
//         res.push_str(&u322hex(self.pc));

//         res
//     }

//     fn p_rsp(&self, ind: usize) -> String {
//         u322hex(self.reg[ind])
//     }

//     fn m_rsp(&self, start: usize, size: usize) -> String {
//         let mut res = String::new();
//         for byte in &self.mem[start..(start+size)] {
//             res.push_str(&u82hex(*byte));
//         }

//         res
//     }
// }

// fn test_one_file(filename: &String, mut steps: i32) {
//     println!("start read {filename}");
//     match bin_file::read_file(filename) {
//         Ok(bytes) => {
//             let mut soc = config::build_soc("rv32im.cfg".to_owned());
//             soc.fill_mem(0, bytes, 0);

//             loop {
//                 if steps >= 0 {
//                     while steps > 0 {
//                         soc.tick();
//                         steps -= 1;
//                     }

//                     let mut key = String::new();
//                     match std::io::stdin().read_line(&mut key) {
//                         Ok(_) => {
//                             // println!("{n} bytes read.");
//                             // println!("key = {}.", key.trim());
//                             let cmds = crate::utils::split_string(key);
//                             if cmds.len() > 0 {
//                                 if cmds[0] == "q" {
//                                     break;
//                                 } else if cmds[0] == "n" {
//                                     if cmds.len() > 1 {
//                                         steps = crate::utils::parse_i32_err_to_min(&cmds[1]);
//                                     } else {
//                                         steps = 1;
//                                     }
//                                 } else if cmds[0] == "r" {
//                                     steps = 1;
//                                 } else if cmds[0] == "i" {
//                                     println!("insert breakpoint.");
//                                     steps = 0;
//                                 } else if cmds[0] == "p" {
//                                     if cmds.len() > 2 {
//                                         soc.print_d(&cmds[1], &cmds[2]);
//                                     } else {
//                                         println!("e.g. p cpu0 reg/csr.");
//                                         println!("     p mem address(hex).");
//                                         println!("     p gpio_a offset(hex).");
//                                     }
//                                     steps = 0;
//                                 } else if cmds[0] == "s" {
//                                     if cmds.len() > 3 {
//                                         soc.set_v_d(&cmds[1], &cmds[2], &cmds[3]);
//                                     } else {
//                                         println!("e.g. s cpu0 index(hex, reg<32, else csr) vvv(hex).");
//                                         println!("     s mem address(hex) vvv(hex).");
//                                         println!("     s gpio_a(perips) address(hex) vvv(hex).");
//                                     }
//                                     steps = 0;
//                                 } else {
//                                     println!("command can not found.");
//                                     steps = 0;
//                                 }
//                             } else {
//                                 println!("command can not found.");
//                                 steps = 0;
//                             }
//                         },
//                         Err(e) => {
//                             println!("input error {e}.")
//                         },
//                     }
//                 } else {
//                     soc.tick();
//                 }
//             }


//             println!("{filename} test completed!!! tick cnt: {}.", soc.get_tick());
//         },
//         Err(e) => {
//             println!("文件读取错误, {}", e);
//         }
//     }
// }

fn main() {
    let args:Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        gdbserver::server_start(&args[1]);
    } else {
        println!("Please input with following format:");
        println!("test file: zemulator filename.");
        println!("--------------------------------");
    }
}
