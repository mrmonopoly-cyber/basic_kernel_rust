use super::io;

pub enum Level
{
    Info,
    Warning,
    Error
}

pub struct DebugLog
{
    level: Level,
}

const COM1 : u16 = 0x3F8;

impl DebugLog{
    pub fn new(level: Level) -> Self {
        Self{level}
    }

    fn write_serial(&self, str: &str)
    {
        for b in str.bytes(){
            io::putb(COM1, b);
        }
    }
}

impl Default for DebugLog{
    fn default() -> Self {
        Self { level: Level::Info }
    }
}

impl core::fmt::Write for DebugLog{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let level = self.level.to_string();

        self.write_serial("[");
        self.write_serial(level);
        self.write_serial("]: ");
        self.write_serial(s);

        Ok(())
    }
}

impl Level {
    pub fn to_string(&self) -> &str{
        match self {
            Level::Info => "info",
            Level::Warning => "warning",
            Level::Error => "error",
        }
        
    }
    
}
