use core::time;
use std::{
    io::Read,
    process::{Command, Stdio}, thread,
};

const INJECTED_JS_PLAY: &'static str = "document.getElementsByClassName('play-btn')[0].click();";
const INJECTED_JS_WOOP_SOUND: &'static str = r###"
var sound = new Audio("https://www.myinstants.com/media/sounds/woo_htcxajK.mp3");
sound.volume = 1;
sound.play();
document.addEventListener("click", function(evt) {
    sound.play();
    setTimeout(() => {
        sound.play();
    }, 50);
});
"###;
const INJECTED_JS_REMOVE_ADS: &'static str = "document.getElementsByClassName('content-container content')[0].remove();";
const INJECTED_JS_WIDEN_CONTAINER: &'static str = "document.getElementsByClassName('play')[1].setAttribute('style', 'height: 100% !important; width: 100% !important');";
const INJECTED_JS_LOAD_BILLY: &'static str = "document.getElementsByClassName('play-logo')[0].style='background-image: url(https://streamsentials.com/wp-content/uploads/2021/01/gachibass.gif); transform: translateX(30%) translateY(180px) scale(3.25);'";
const INJECTED_JS_CLICK_OW: &'static str = "document.getElementById('game-nav-btn-Pro').click();";
const CEF_PATH: &'static str = ".\\cefdebug.exe";

fn main() {
    fn launch_bnet_debug() {
        // For some reason, running the commands that are in launch.bat would result in "File not found" exceptions
        // if I were to do so via Rust's `Command`. So they're in a batch file.
        Command::new(".\\launch.bat").output().expect(":|");
        println!("Launched, waitng.");
        thread::sleep(time::Duration::from_secs(10));
    }

    fn inject_js(js: &str, debugger_url: &str) {
        Command::new(CEF_PATH)
            .args(&["--url", debugger_url, "--code", js])
            .spawn()
            .expect("nope");
    }

    fn change_bg(debugger_url: &str) {
        inject_js(
            r###"document.getElementsByTagName('body')[0].setAttribute('style', "background-image: url('https://media.tenor.com/x8v1oNUOmg4AAAAd/rickroll-roll.gif'); background-size: cover;");"###,
            debugger_url
        );
        inject_js(INJECTED_JS_LOAD_BILLY, debugger_url);
        inject_js(INJECTED_JS_REMOVE_ADS, debugger_url);
        inject_js(INJECTED_JS_WIDEN_CONTAINER, debugger_url);
        inject_js(INJECTED_JS_WOOP_SOUND, debugger_url);
    }

    launch_bnet_debug();

    // This is incredibly slow. Takes 10 to 20 seconds to find all node debuggers.
    let cef_scan_output = Command::new(CEF_PATH)
        .stderr(Stdio::piped())
        .spawn()
        .expect("nope");

    // CEFDebug, for some reason, outputs everything to stderr.
    let mut stderrout: String = String::new();
    cef_scan_output
        .stderr
        .unwrap()
        .read_to_string(&mut stderrout)
        .unwrap();

    for lines in stderrout.split("\n") {
        let filtered_lines: Vec<&str> =
            lines.split("U: ").filter(|p| p.contains("ws://")).collect();
        for debugger_url in filtered_lines.iter() {
            let debugger_url: Vec<&str> = debugger_url
                .split("U: ")
                .next()
                .unwrap()
                .split("\r")
                .collect();

            inject_js(INJECTED_JS_CLICK_OW, debugger_url[0]);
            change_bg(debugger_url[0]);
            // inject_js(INJECTED_JS_PLAY, debugger_url[0]);
        }
    }
}
