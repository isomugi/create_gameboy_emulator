impl Cpu{
    pub fn nop(&mut self, bus: &Peripherais){
        self.fetch(bus);
    }
}