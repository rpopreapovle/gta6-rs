//! GTA VI — Rust Edition. All the crime, none of the gigabytes.
use std::thread::sleep;
use std::time::Duration;

struct Player { name: String, wanted: u8, cash: i64 }

impl Player {
    fn new(name: &str) -> Self { Self { name: name.into(), wanted: 0, cash: 0 } }
    fn commit_crime(&mut self, what: &str, payout: i64) {
        self.wanted = (self.wanted + 1).min(5);
        self.cash += payout;
        println!("  ! {} -> {} (+${})  wanted: {}/5", self.name, what, payout, self.wanted);
    }
    fn stars(&self) -> String { "*".repeat(self.wanted as usize) + &".".repeat(5 - self.wanted as usize) }
}

fn loading() {
    let steps = ["Vice City", "Leonida", "ray tracing", "shaders", "physics", "NPC schedules", "story"];
    for (i, s) in steps.iter().enumerate() {
        print!("\r[{:>3}%] compiling {}...", (i + 1) * 100 / steps.len(), s);
        use std::io::Write; std::io::stdout().flush().ok();
        sleep(Duration::from_millis(120));
    }
    println!("\r[100%] ready. 0 bytes of assets, 1 binary.            ");
}

fn main() {
    println!("=== GRAND THEFT AUTO VI — RUST EDITION (0.0.1-alpha.0) ===");
    loading();
    let mut p = Player::new("Lucia");
    println!("\nMission: \"Two of Us\"");
    p.commit_crime("stole a car", 250);
    p.commit_crime("robbed a gas station", 1800);
    p.commit_crime("escaped the cops", 0);
    println!("\n  {}  cash: ${}  wanted: {}", p.name, p.cash, p.stars());
    println!("\nTODO: everything else. PRs welcome (MIT).");
}
