use rand::Rng;
use std::process::Command;
use std::process::Stdio;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};

static NUMBER: AtomicUsize = AtomicUsize::new(20360);

pub async fn print_code() {
    let num = NUMBER.fetch_add(1, Ordering::SeqCst);
    let duration = time::Duration::from_millis(1000);
    let content = &receipt_build(num);
    let command = format!(r#"echo "{}" | cat | sudo lp -d Deli_DL-581P -o raw"#, content);
    let shell = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .spawn();
    println!("{}", shell.unwrap().id());
    thread::sleep(duration);
}

fn gen_rand() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(4..100)
}

fn receipt_build(index: usize) -> String {
    let number = format!("{}", index);
    let weeks: usize = gen_rand();
    let mut receipt: String = String::new();
    receipt.push_str("          ---=== <3 ===---\n");
    receipt.push_str("\n\n");
    receipt.push_str("        Congratulations!\n");
    receipt.push_str("\n");
    receipt.push_str("        Your application\n");
    receipt.push_str("       has been submitted.\n\n");
    receipt.push_str("\n\n\n");
    receipt.push_str("Your unique reference number is\n\n\n");
    receipt.push_str("             ");
    receipt.push_str(&number);
    receipt.push_str("\n\n\n");
    receipt.push_str("    We are currently processing\n");
    receipt.push_str("       many applications.\n\n");
    receipt.push_str(" Your current waiting time is ");
    receipt.push_str("   up to   ");
    receipt.push_str(&weeks.to_string());
    receipt.push_str("    weeks.");
    receipt.push_str("\n\n\n");
    receipt.push_str("          ---=== <3 ===---\n");
    receipt.push_str("\n\n\n");
    receipt
}

pub async fn print_invitation() {
    let duration = time::Duration::from_millis(2000);
    let content = invitation_build();
    let command = format!(r#"echo "{}" | cat | sudo lp -d Deli_DL-581P -o raw"#, content);
    // for _ in 0..3 {
    //     let shell = Command::new("sh")
    //         .arg("-c")
    //         .arg(&command)
    //         .stdout(Stdio::piped())
    //         .spawn();
    //     println!("{}", shell.unwrap().id());
    //     thread::sleep(duration);
    // }
    let shell = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .spawn();
    println!("{}", shell.unwrap().id());
    thread::sleep(duration);
}

fn invitation_build() -> String {
    //     ~~ <3 ~~
    //
    // Under Review
    // It is finally happening!
    //
    // Whereby invite you to your wedding:
    //
    // on Saturday,
    // the 20th of September 2025
    // from 15:00 until 20:00,
    //
    // at the Music Pavillion,
    // at Festplassen,
    // Olav Kyrres gate 27,
    // 5014 Bergen
    //
    // In order to acquire the benefits
    // of marriage you will need to:
    //
    // 1. Stand together in pairs.
    // This can be anyone you choose.
    //
    // 2. Join the line and enter the
    // Pavillion when it is your turn.
    //
    // 3. When you reach the stand,
    // do what it asks of you.
    //
    // 4. A receipt will print out. Take it.
    //
    // 5. Congratulations. You will be celebrated.
    //
    // 6. You may take a slim slice of cake,
    // and join the party.
    //
    // 7. You may try again at another time or
    // with someone else.
    //
    // We hope you find the time.
    //
    // Warmly,
    // Mila and Pouria
    //
    // ~~ <3 ~~
    let mut invitation: String = String::new();
    invitation.push_str("          --== <3 ==--\n");
    invitation.push_str("\n\n");
    invitation.push_str("         Under Review\n");
    invitation.push_str("   It is finally happening!\n");
    invitation.push_str("\n");
    invitation.push_str("      We hereby invite you to\n");
    invitation.push_str("          your wedding:\n");
    invitation.push_str("\n");
    invitation.push_str("         on Saturday,\n");
    invitation.push_str("   the 20th of September 2025\n");
    invitation.push_str("    from 15:00 until 18:00,\n");
    invitation.push_str("\n");
    invitation.push_str("    at the Music Pavillion,\n");
    invitation.push_str("         at Festplassen,\n");
    invitation.push_str("     Olav Kyrres gate 27,\n");
    invitation.push_str("          5014 Bergen\n");
    invitation.push_str("\n");
    invitation.push_str("In order to acquire the benefits\n");
    invitation.push_str("of marriage you will need to:\n");
    invitation.push_str("\n");
    invitation.push_str("1. Come alone or with others.\n");
    invitation.push_str("\n");
    invitation.push_str("2. Stand together in pairs.\n");
    invitation.push_str("This can be anyone you choose.\n");
    invitation.push_str("\n");
    invitation.push_str("3. Join the line and enter the\n");
    invitation.push_str("Pavillion when it is your turn.\n");
    invitation.push_str("\n");
    invitation.push_str("4. When you reach the stand,\n");
    invitation.push_str("do what it asks of you.\n");
    invitation.push_str("\n");
    invitation.push_str("5. A receipt will print out.\n");
    invitation.push_str("Take it.\n");
    invitation.push_str("\n");
    invitation.push_str("6. Congratulations.\n");
    invitation.push_str("You will be celebrated.\n");
    invitation.push_str("\n");
    invitation.push_str("7. You may take a slim slice of cake, and join the party.\n");
    invitation.push_str("\n");
    invitation.push_str("8. You may try again at another\n");
    invitation.push_str("time or with someone else.\n");
    invitation.push_str("\n");
    invitation.push_str("   We hope you find the time.\n");
    invitation.push_str("\n");
    invitation.push_str("           Warmly,\n");
    invitation.push_str("       Mila and Pouria\n");
    invitation.push_str("          (and Liu)\n");
    invitation.push_str("\n\n\n");
    invitation.push_str("          --== <3 ==--\n\n\n\n\n");
    invitation
}
