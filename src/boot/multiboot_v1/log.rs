pub struct VGALog
{
    vga_color: u8,
    x:u8,
    y:u8,
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

#[allow(unused)]
impl VGALog {
    const VGA_BUFFER_ADDRESS: usize = 0xB8000;
    const VGA_WIDTH : u8= 80;
    const VGA_HEIGHT : u8 = 25;


    pub fn initialize(foreground_color: VgaColor, background_color: VgaColor) -> Self {
        let vga_color = Self::vga_entry_color(foreground_color, background_color);

        for y in 0..Self::VGA_HEIGHT{
            for x in 0..Self::VGA_WIDTH{
                Self::write_c_at(b' ', y, x, vga_color);
            }
        }

        Self{vga_color,x:0,y:0}
    }

    pub fn write_string(&mut self, string: &str)
    {
        for c in string.bytes()
        {
            self.putc(c);
        }
    }

    fn vga_entry_color(foreground_color: VgaColor, background_color: VgaColor) -> u8
    {
        (foreground_color as u8) | (background_color as u8) << 4
    }

    pub fn putc(&mut self, c:u8)
    {
        unsafe {
            if c ==b'\n'
            {
                self.x =0;
                self.y+=1;
                if self.y == Self::VGA_HEIGHT
                {
                    self.y=0;
                }
            }
            else
            {
                Self::write_c_at(c, self.y, self.x, self.vga_color);
                self.x+=1;
                if self.x >= Self::VGA_WIDTH
                {
                    self.x=0;
                    self.y+=1;
                    if self.y>= Self::VGA_HEIGHT
                    {
                        self.y=0;
                    }
                }
            }
        }
    }

    fn vga_entry(uc: u8, color: u8) -> u16 
    {
        let uc = u16::from(uc);
        let color = u16::from(color);
        uc | color << 8
    }


    fn write_c_at(c:u8, pos_y:u8, pos_x:u8, color: u8)
    {
        let index : usize = usize::from(pos_y * Self::VGA_WIDTH + pos_x);
        let vga_ptr : *mut u16 = Self::VGA_BUFFER_ADDRESS as *mut u16;
        let vga_entry = Self::vga_entry(c, color);
        unsafe{
            vga_ptr.add(index).write(vga_entry);
        }
    }
}
