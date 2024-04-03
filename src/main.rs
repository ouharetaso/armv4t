#[allow(dead_code)]


mod armv4t;
use armv4t::*;

struct MyMemory{
    memory: [u8; 0x10000],
}


impl Bus for MyMemory{
    fn access(&mut self, addr: u32, data: &mut u32, r: BusRW) -> Result<u32, ()>{
        match r{
            BusRW::Read => {
                *data = self.memory[addr as usize] as u32 + 
                ((self.memory[(addr + 1) as usize] as u32) << 8) + 
                ((self.memory[(addr + 2) as usize] as u32) << 16) + 
                ((self.memory[(addr + 3) as usize] as u32) << 24);
            }
            BusRW::Write => {
                self.memory[addr as usize] = (*data & 0xFF) as u8;
                self.memory[(addr + 1) as usize] = ((*data >> 8) & 0xFF) as u8;
                self.memory[(addr + 2) as usize] = ((*data >> 16) & 0xFF) as u8;
                self.memory[(addr + 3) as usize] = ((*data >> 24) & 0xFF) as u8;
            }
        }
        return Ok(0);
    }
}

impl MyMemory{
    fn new() -> MyMemory{
        MyMemory{
            memory: [0; 0x10000],
        }
    }
    fn load(&mut self, addr: u32, data: u8){
        self.memory[addr as usize] = data;
    }
}


fn main() {
    let mut mem = MyMemory::new();
    let filename = "program.bin";
    let program = std::fs::read(filename).unwrap();
    for i in 0..program.len(){
        mem.load(i as u32, program[i]);
    }
    // _ =mem.access(0, &mut 0xe280002a, BusRW::Write);
    let mut cpu = Cpu::<MyMemory>::new(mem);
    cpu.reset();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
}
