//! Read-only local XPM import audit; emits counts, never copies samples.
use loop_club::xpm::Program;
use std::path::PathBuf;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let path = PathBuf::from(
        args.next()
            .ok_or("Usage: xpm_inspect PROGRAM.xpm [SCOPE]")?,
    );
    let scope = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| path.parent().unwrap().to_path_buf());
    let program = Program::open(&path, &scope)?;
    for bank in &program.banks {
        let j = program.prepare(*bank, 0, 44100)?;
        let meta = j.kit.xpm.as_ref().unwrap().meta;
        println!(
            "{}",
            serde_json::json!({"bank":(b'A'+*bank as u8)as char,"populated_pads":meta.pads.iter().filter(|p|p.count>0).count(),"zones":meta.pads.iter().map(|p|usize::from(p.count)).sum::<usize>(),"bytes":j.kit.bytes(),"native_fx_omitted":program.native_fx})
        );
    }
    Ok(())
}
