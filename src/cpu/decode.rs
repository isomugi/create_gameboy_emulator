impl Cpu{
    pub fn decode(&mut self,bus: &mut Peripherais){
        match self.ctx.opcode{
            0x00 => self.nop(bus),
            
            _    => panic!("Not implemented: {:02x}", self.opcode),
        }
    }
}