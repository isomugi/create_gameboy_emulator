#[derive(Default)]
struct Ctx{
    opcode: u8,
    cb: bool,
}
pub struct Cpu{
    regs: Registers,
    ctx: Ctx,
}

pub fn emulate_cycle(&mut self, bus: &mut Peripherais){
    self.decode(bus);
}