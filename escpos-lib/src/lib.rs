use serialport::SerialPort;

use std::io::Result as IoResult;

mod cmds;
mod format;

use cmds::{CutMode, EscPosCmd};
pub use format::{FmtStr, FormattedStr};

/// Special characters
mod chars {
    pub const ESC: char = '\x1b';
    pub const LF: char = '\x0a';
    pub const GS: char = '\x1d';
    pub const INITIALIZE_PRINTER: char = '@';
}

pub struct Printer<P>
where
    P: SerialPort,
{
    port: P,
}

impl<P> Printer<P>
where
    P: SerialPort,
{
    pub fn new(port: P) -> IoResult<Self> {
        let mut printer = Printer { port };
        printer.exec(EscPosCmd::InitializePrinter)?;
        Ok(printer)
    }

    pub fn print_test_page(&mut self) -> IoResult<()> {
        let header = format!("{}\nDies ist ein Test\n", " TEST ".reverse());
        let format_strings = vec![
            "Emphasized".emph(),
            "Higher".higher(),
            "Wider".wider(),
            "Underlined".underline(),
            "Reversed".reverse(),
            "Small".small(),
            "Emph Higher".emph().higher(),
            "Emph Wider".emph().wider(),
            "Emph Underlined".emph().underline(),
            "Emph Reversed".emph().reverse(),
            "Emph Small".emph().small(),
            "Higher Wider".higher().wider(),
            "Higher Underlined".higher().underline(),
            "Higher Reversed".higher().reverse(),
            "Higher Small".higher().small(),
            "Wider Underlined".wider().underline(),
            "Wider Reversed".wider().reverse(),
            "Wider Small".wider().small(),
            "Underlined Reversed".underline().reverse(),
            "Underlined Small".underline().small(),
            "Reversed Small".reverse().small(),
        ];
        self.write(header)?;
        for string in format_strings {
            self.write(&format!(" - {}\n", string))?
        }
        self.write(&format!("\n{}\n", "CHARS".wider()))?;
        let numbers = "0123456789";
        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let german = "äöüß";
        for c in numbers.chars() {
            self.write(&format!(" {}", c))?;
        }
        self.write("\n")?;
        for c in chars.chars() {
            self.write(&format!(" {}", c))?;
        }
        self.write("\n")?;
        for c in german.chars() {
            self.write(&format!(" {}", c))?;
        }
        self.write("\n")?;
        self.exec(EscPosCmd::PrintAndFeedLines(4))?;
        self.exec(EscPosCmd::CutPaper(CutMode::Full))?;
        Ok(())
    }

    pub fn write<S: Into<String>>(&mut self, text: S) -> IoResult<()> {
        write!(self.port, "{}", EscPosCmd::Text(&text.into()))
    }

    pub fn write_and_cut<S: Into<String>>(&mut self, text: S) -> IoResult<()> {
        self.write(text)?;
        self.exec(EscPosCmd::PrintAndFeedLines(4))?;
        self.exec(EscPosCmd::CutPaper(CutMode::Full))
    }

    pub fn exec(&mut self, cmd: EscPosCmd) -> IoResult<()> {
        write!(self.port, "{}", cmd)
    }
}

/// Escape a string to print safely
pub fn escape(raw: &str) -> String {
    raw.replace(chars::ESC, "?").replace(chars::GS, "?")
}
