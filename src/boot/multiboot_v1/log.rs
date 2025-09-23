pub struct VGALog
{
    vga_color: u8,
}

/* Hardware text mode color constants. */
#[repr(u8)]
#[allow(unused)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

static mut VGA_POS_X : u8=0;
static mut VGA_POS_Y : u8=0;

#[allow(unused)]
impl VGALog {
    const VGA_BUFFER_ADDRESS: usize = 0xB8000;
    const VGA_WIDTH : u8= 2;
    const VGA_HEIGHT : u8 = 2;


    pub fn initialize(foreground_color: VgaColor, background_color: VgaColor) -> Self {
        let vga_color = Self::vga_entry_color(foreground_color, background_color);

        for y in 0..Self::VGA_HEIGHT{
            for x in 0..Self::VGA_HEIGHT{
                Self::write_c_at(' ', y, x, vga_color);
            }
        }

        Self{vga_color}
    }

    pub fn write_string(&self, string: &str)
    {
        for c in string.chars()
        {
            self.putc(c);
        }
    }

    fn vga_entry_color(foreground_color: VgaColor, background_color: VgaColor) -> u8
    {
        (foreground_color as u8) | (background_color as u8) << 4
    }

    pub fn putc(&self, c:char)
    {
        unsafe {
            if c=='\n'
            {
                VGA_POS_X=0;
                VGA_POS_Y+=1;
                if VGA_POS_Y == Self::VGA_HEIGHT
                {
                    VGA_POS_Y =0;
                }

            }
            else
            {
                Self::write_c_at(c, VGA_POS_Y, VGA_POS_X, self.vga_color);
                VGA_POS_X+=1;
                if VGA_POS_X == Self::VGA_WIDTH
                {
                    VGA_POS_X=0;
                    VGA_POS_Y+=1;
                    if VGA_POS_Y == Self::VGA_HEIGHT
                    {
                        VGA_POS_Y=0;
                    }
                }
            }
        }
    }

    fn vga_entry(uc: u8, color: u8) -> u16 
    {
        (uc as u16) | ((color as u16) << 8)
    }


    fn write_c_at(c:char, pos_y:u8, pos_x:u8, color: u8)
    {
        let index : usize = (pos_y as usize) * (Self::VGA_WIDTH as usize) + (pos_x as usize);
        let vga_ptr : *mut u16 = Self::VGA_BUFFER_ADDRESS as *mut u16;
        let vga_entry = Self::vga_entry(c as u8, color);
        unsafe{
            vga_ptr.add(index).write(vga_entry);
        }
    }
}

impl log::Log for VGALog{
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        todo!()
    }

    fn log(&self, record: &log::Record) {
        // self.write_string(record.level().as_str());
        // self.write_string(":");
        // match record.args().as_str(){
        //     Some(s) => self.write_string(s),
        //     None => self.write_string("\n"),
        // }
    }

    fn flush(&self) {
        todo!()
    }
    // add code here
}
