
use std::net::{TcpListener, TcpStream};
use std::{fs, thread};
use std::io::{Read, Write, Error};

static mut NO_ACK_MODE: bool = false;

fn hex2u8(hex: &str) -> u8 {
    match u8::from_str_radix(hex, 16) {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

fn hex2usize(hex: &str) -> usize {
    match usize::from_str_radix(hex, 16) {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

fn u82hex(u: u8) -> String {
    format!("{:02x}", u)
}

fn u322hex(u: u32) -> String {
    let u0 = u as u8;
    let u1 = u>>8 as u8;
    let u2 = u>>16 as u8;
    let u3 = u>>24 as u8;
    format!("{:02x}{:02x}{:02x}{:02x}", u0, u1, u2, u3)
}

fn str_add_sum(s: &str) -> u8 {
    let mut sum: u32 = 0;
    for c in s.bytes() {
        sum += c as u32;
    }
    // 限制校验码为8位（0-255）
    sum as u8
}

fn pack_rsp(s: &str) -> String {
    let mut res = String::new();

    if unsafe { NO_ACK_MODE } {
        res.push_str("$");
    } else {
        res.push_str("+$");
    }
    res.push_str(s);
    res.push('#');
    let sum = str_add_sum(s);
    res.push_str(&u82hex(sum));

    res
}

pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let byte_content = fs::read(path)?;
    Ok(byte_content)
}

struct Emu {
    pc: u32,
    reg: [u32; 32],
    mem: Vec<u8>,
}

impl Emu {
    fn new() -> Self {
        match read_file("exam.bin") {
            Ok(bytes) => Emu{ pc: 0, reg: [0; 32], mem: bytes},
            Err(_) => {
                println!("file read failed.");
                Emu{ pc: 0, reg: [0; 32], mem: vec![0; 8192] }
            },
        }
    }

    fn s_rsp(&mut self) -> String {
        self.pc += 4;
        "S00".to_string()
    }

    fn g_rsp(&self) -> String {
        let mut res = String::new();
        for r in self.reg {
            res.push_str(&u322hex(r));
        }
        res.push_str(&u322hex(self.pc));

        res
    }

    fn p_rsp(&self, ind: usize) -> String {
        u322hex(self.reg[ind])
    }

    fn m_rsp(&self, start: usize, size: usize) -> String {
        let mut res = String::new();
        for byte in &self.mem[start..(start+size)] {
            res.push_str(&u82hex(*byte));
        }

        res
    }
}

fn handle_rsp(emu: &mut Emu, in_str: String) -> Option<String> {
    
    let ss_str: Vec<&str> = in_str.split(&['$','#']).collect();
    if ss_str.len() >= 3 {
        let sum = str_add_sum(ss_str[1]);
        let sum2 = hex2u8(&ss_str[2][0..2]);
        println!("sum check: {sum} .. {sum2}");
        if sum == sum2 {
            let out_str;
            if ss_str[1].starts_with("qSupported") {
                out_str = pack_rsp("PacketSize=1024;hwbreak+;QStartNoAckMode+");
            } else if ss_str[1].eq("QStartNoAckMode") {
                out_str = pack_rsp("OK");
                unsafe { NO_ACK_MODE = true };
            } else if ss_str[1].eq("?") {
                out_str = pack_rsp("S05");
            } else if ss_str[1].eq("qAttached") {
                out_str = pack_rsp("1");
            } else if ss_str[1].eq("g") {
                out_str = pack_rsp(&emu.g_rsp());
            } else if ss_str[1].starts_with("p") {
                let index = hex2usize(&ss_str[1][1..]);
                println!("index:{index}");
                out_str = pack_rsp(&emu.p_rsp(index));
            } else if ss_str[1].starts_with("m") {
                let start_size: Vec<&str> = ss_str[1][1..].split(',').collect();
                if start_size.len() == 2 {
                    let start = hex2usize(start_size[0]); 
                    let size = hex2usize(start_size[1]);
                    println!("start:{start}, size{size}");
                    out_str = pack_rsp(&emu.m_rsp(start, size));
                } else {
                    out_str = pack_rsp("");
                }
            } else if ss_str[1].starts_with("vCont") {
                out_str = pack_rsp("");
            } else if ss_str[1].eq("s") {
                out_str = pack_rsp(&emu.s_rsp());
            } else {
                out_str = pack_rsp("");
            }
            println!("out: {out_str}");
            return Some(out_str);
        }
    }

    None
}

// Handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    let mut emu = Emu::new();
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { 
            println!("read 0, return.");
            return Ok(()); 
        }
        match String::from_utf8(buf[0..bytes_read].to_vec()) {
            Ok(in_str) => {
                println!("in: {in_str}");
                match handle_rsp(&mut emu, in_str) {
                    Some(out_str) => {
                        stream.write(out_str.as_bytes()).unwrap();
                    }
                    None => println!("no response need."),
                };
            }
            Err(e) => println!("from utf8 error {e}."),
        }
    }
}

fn main() {
    // let str1 = "+$qSupported:multiprocess+;swbreak+;hwbreak+;qRelocInsn+;fork-events+;vfork-events+;exec-events+;vContSupported+;QThreadEvents+;no-resumed+;xmlRegisters=i386#6a";
    // // split_str(str1.to_owned());
    // let str1 = str1.to_owned();
    // let new_str: Vec<&str> = str1.split(&['$','#']).collect();
    // if new_str.len() >= 3 {
    //     let sum = str_add_sum(new_str[1]);
    //     let sum2 = hex2u8(new_str[2]);
    //     println!("{sum} .. {sum2}");
    // }

    let listener = TcpListener::bind("0.0.0.0:3333")
    .expect("Could not bind");
    for stream in listener.incoming() {
    match stream {
        Err(e) => { eprintln!("failed: {}", e) }
        Ok(stream) => {
            thread::spawn(move || {
                handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
