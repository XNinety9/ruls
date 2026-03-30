use ansi_term::{Style, Color};

pub struct Theme {
    pub dir:        Style,
    pub symlink:    Style,
    // pub bkn_symlink:Style,
    pub executable: Style,
    pub file:       Style,

    pub size_none:         Style,
    pub size_bytes:        Style,
    pub size_kbytes:       Style,
    pub size_mbytes:       Style,
    pub size_gbytes:       Style,
    pub size_tbytes:       Style,
    
    pub date:              Style,

    pub perm_r_user:       Style,
    pub perm_w_user:       Style,
    pub perm_x_user:       Style,
    pub perm_r_group:      Style,
    pub perm_w_group:      Style,
    pub perm_x_group:      Style,
    pub perm_r_other:      Style,
    pub perm_w_other:      Style,
    pub perm_x_other:      Style,
    pub perm_dash:         Style,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            dir:            Color::Blue.bold(),
            symlink:        Color::Cyan.normal(),
            // bkn_symlink:    Color::Red.normal(),
            executable:     Color::Green.bold(),
            file:           Style::new(),

            size_none:      Color::RGB(100, 100, 100).bold(),            
            size_bytes:     Color::Green.normal(),
            size_kbytes:    Color::Green.bold(),
            size_mbytes:    Color::Yellow.normal(),
            size_gbytes:    Color::Red.normal(),
            size_tbytes:    Color::Purple.normal(),

            date:           Color::Blue.normal(),

            perm_r_user:    Color::Yellow.bold(),
            perm_w_user:    Color::Red.bold(),
            perm_x_user:    Color::Green.bold().underline(),
            perm_r_group:   Color::Yellow.normal(),
            perm_w_group:   Color::Red.normal(),
            perm_x_group:   Color::Green.normal().underline(),
            perm_r_other:   Color::Yellow.normal(),
            perm_w_other:   Color::Red.normal(),
            perm_x_other:   Color::Green.normal().underline(),
            perm_dash:      Color::White.bold(),
        }
    }
}
