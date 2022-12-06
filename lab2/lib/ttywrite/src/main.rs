mod parsers;

use serial;
use structopt;
use structopt_derive::StructOpt;
use xmodem::Xmodem;

use xmodem::{Progress};

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader};

    let opt = Opt::from_args();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");
    (&mut port).set_timeout(Duration::new(opt.timeout, 0)).expect("failed to set timeout");
    let mut settings = (&port).read_settings().expect("failed to read settings");
    (&mut settings).set_baud_rate(opt.baud_rate).expect("failed to set baud rate");
    (&mut settings).set_char_size(opt.char_width);
    (&mut settings).set_flow_control(opt.flow_control);
    (&mut settings).set_stop_bits(opt.stop_bits);
    (&mut port).write_settings(&settings).expect("failed to write settings");


    let mut to = port;
    let mut buf_reader = None;
    if let Some(input_path_buf) = opt.input {
        let file = File::open(input_path_buf).expect("invalid file path");
        buf_reader = Some(BufReader::new(file));
    }

    if opt.raw {
        let num_bytes;
        if let Some(mut reader) = buf_reader {
            num_bytes = io::copy(&mut reader, &mut to).expect("raw transmission failed");
        } else {
            num_bytes = io::copy(&mut io::stdin(), &mut to).expect("raw transmission failed");
        }
        println!("Done: {} bytes written in total", num_bytes);
    } else {
        let num_bytes;
        if let Some(reader) = buf_reader {
            num_bytes = Xmodem::transmit_with_progress(reader, to, progress_fn)
                .expect("xmodem transmission failed");
        } else {
            num_bytes = Xmodem::transmit_with_progress(io::stdin(), to, progress_fn)
            .expect("xmodem transmission failed");
        }
        println!("Done: {} bytes written in total", num_bytes);
    }
}
