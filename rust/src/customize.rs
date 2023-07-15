mod room_palettes;
mod samus_sprite;

use room_palettes::apply_area_themed_palettes;
use crate::{
    game_data::GameData,
    patch::{Rom, snes2pc},
};
use anyhow::Result;

pub use self::samus_sprite::SamusSpriteCustomizer;

#[derive(Debug)]
pub struct CustomizeSettings {
    pub area_themed_palette: bool,
    pub disable_music: bool,
    pub disable_beeping: bool,
}

fn remove_mother_brain_flashing(rom: &mut Rom) -> Result<()> {
    // Disable start of flashing after Mother Brain 1:
    rom.write_u16(snes2pc(0xA9CFFE), 0)?;

    // Disable end of flashing (to prevent palette from getting overwritten)
    rom.write_u8(snes2pc(0xA9D00C), 0x60)?;  // RTS

    Ok(())
}


pub fn customize_rom(
    rom: &mut Rom,
    seed_patch: &[u8],
    samus_sprite: &Option<Vec<u8>>,
    settings: &CustomizeSettings,
    game_data: &GameData,
    samus_customizer: &SamusSpriteCustomizer,
) -> Result<()> {
    rom.resize(0x400000);
    let patch = ips::Patch::parse(seed_patch).unwrap();
    // .with_context(|| format!("Unable to parse patch {}", patch_path.display()))?;
    for hunk in patch.hunks() {
        rom.write_n(hunk.offset(), hunk.payload())?;
    }
    remove_mother_brain_flashing(rom)?;
    if settings.area_themed_palette {
        apply_area_themed_palettes(rom, game_data)?;
    }
    if settings.disable_music {
        rom.write_u8(snes2pc(0xcf8413), 0x6F)?;
    }
    if settings.disable_beeping {
        rom.write_n(snes2pc(0x90EA92), &[0xEA; 4])?;
        rom.write_n(snes2pc(0x90EAA0), &[0xEA; 4])?;
        rom.write_n(snes2pc(0x90F33C), &[0xEA; 4])?;
        rom.write_n(snes2pc(0x91E6DA), &[0xEA; 4])?;
    }
    if let Some(x) = samus_sprite {
        samus_customizer.apply(rom, x)?;
    }
    Ok(())
}
