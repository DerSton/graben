#[repr(u16)]
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum RecordType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    ANY = 255,
}
