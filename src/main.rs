use core::time;
use std::{
    io::Read,
    process::{Command, Stdio},
    thread,
};

// This launches Overwatch. Which was the initial reason I did this, to be able to send a packet to launch the game.
// Then, well then Billy took the wheel.
const INJECTED_JS_PLAY: &'static str = "document.getElementsByClassName('play-btn')[0].click();";

const INJECTED_JS_WOOP_SOUND: &'static str = r###"console.log("WOOP_SOUND"); 
var sound = new Audio("https://www.myinstants.com/media/sounds/woo_htcxajK.mp3");
var hasClicked = false;
document.addEventListener("click", function(evt) {
    sound.volume = 1;
    sound.play();
    if (!hasClicked) {
        ASSTRUMS.play();
        hasClicked = true;
    }
});
"###;
const INJECTED_JS_ALL_BILLY_AVATARS: &'static str = r###"console.log("ALL_BILLY_AVATARS"); 
for (const a of document.getElementsByClassName('name')) {
    a.innerHTML='<b>BILLY HERRINGTON</b>';
}
for (const a of document.getElementsByTagName('img')) {
    a.style = "background-image: url('https://streamsentials.com/wp-content/uploads/2021/01/gachibass.gif')";
}
for (const a of document.getElementsByClassName("avatar-img")) {
    a.style = "background-image: url('https://streamsentials.com/wp-content/uploads/2021/01/gachibass.gif')";
}
"###;
const INJECTED_JS_ALL_BILLY_IMAGES: &'static str = r###"console.log("ALL BILLY IMAGES"); 
var audioel = document.createElement('audio');
audioel.setAttribute('src', 'https://dl.sndup.net/fpwt/Billy%20and%20the%20ASStrums%20-%20HandClap%20(right%20version).mp3');
//audioel.setAttribute('src', 'https://dl.sndup.net/xtjs/[ytmp3page]_Leave_the_Gachimuchi_on_.mp3');
audioel.setAttribute('preload', 'auto');
audioel.setAttribute('id', 'ASSTRUMS');
document.getElementById('main-header').appendChild(audioel);
"###;
const INJECTED_JS_TOGGLE_BILLY_RADIO: &'static str = "console.log('BILLY_RADIO'); var list = document.getElementsByClassName('QuickLink-list');function toggleBillyRadio() {if (!document.getElementById('ASSTRUMS').paused) {audioel.pause();document.getElementById('BILLYRADIO').innerHTML = 'PLAY BILLY RADIO';} else {document.getElementById('BILLYRADIO').innerHTML = 'PAUSE BILLY RADIO';audioel.volume = 1;audioel.play();}}";
const INJECTED_JS_BILLY_RADIO_CTL: &'static str = "console.log('BILLY_RADIO_CTL'); function SUPPORTBILLY() {window.location.href = 'https://twitter.com/Carter_OW';alert('thank u for supporting billy.');}";
const INJECTED_JS_REMOVE_ADS: &'static str = "console.log('REMOVE_ADS'); document.getElementsByClassName('content-container content')[0].remove();";
const INJECTED_JS_WIDEN_CONTAINER: &'static str = "console.log('WIDEN_CONTAINER'); document.getElementsByClassName('play')[1].setAttribute('style', 'height: 100% !important; width: 100% !important');";
const INJECTED_JS_LOAD_BILLY: &'static str = "console.log('LOAD_BILLY'); document.getElementsByClassName('play-logo')[0].style='background-image: url(https://streamsentials.com/wp-content/uploads/2021/01/gachibass.gif); transform: translateX(30%) translateY(180px) scale(3.25);'";
const INJECTED_JS_CLICK_OW: &'static str =
    "console.log('CLICK_OW'); document.getElementById('game-nav-btn-Pro').click();";
const INJECTED_JS_BILLY_BUTTONS: &'static str = r###"console.log('BILLY_BUTTONS'); list[0].innerHTML = `<button id="BILLYRADIO" style="background-color: #fe2ef7; width: 100%; height: 100%; color: white; border-radius: 1.5rem; padding: 2rem;" onclick="toggleBillyRadio()">PAUSE BILLY RADIO</button>`; list[0].innerHTML += `<button style="background-color: #696969; color: gold; border-radius: 2.25rem; border: 2px solid cyan; padding: 3rem;" id="SUPPORTBILLYNET" onclick="SUPPORTBILLY()">SUPPORT BILLY.NET</button>`;"###;
const INJECTED_JS_RICK_BG: &'static str = "document.getElementsByTagName('body')[0].setAttribute('style', 'background-image: url(https://media.tenor.com/x8v1oNUOmg4AAAAd/rickroll-roll.gif); background-size: cover;');";
const CEF_PATH: &'static str = ".\\cefdebug.exe";

fn main() {
    fn launch_bnet_debug() {
        // For some reason, running the commands that are in launch.bat would result in "File not found" exceptions
        // if I were to do so via Rust's `Command`. So they're in a batch file.
        Command::new(".\\launch.bat").output().expect(":|");
        println!("Launched, loading Billy.");
        thread::sleep(time::Duration::from_secs(10));
    }

    fn inject_js(js: &str, debugger_url: &str) {
        Command::new(CEF_PATH)
            .args(&["--url", debugger_url, "--code", js])
            .spawn()
            .expect("nope");
        thread::sleep(time::Duration::from_millis(150));
    }

    fn init_billy(debugger_url: &str) {
        inject_js(INJECTED_JS_RICK_BG, debugger_url);
        inject_js(
            &format!("{} {}", INJECTED_JS_WOOP_SOUND, INJECTED_JS_REMOVE_ADS),
            debugger_url,
        );
        inject_js(INJECTED_JS_ALL_BILLY_IMAGES, debugger_url);
        inject_js(INJECTED_JS_WIDEN_CONTAINER, debugger_url);
        inject_js(INJECTED_JS_LOAD_BILLY, debugger_url);
        inject_js(INJECTED_JS_ALL_BILLY_AVATARS, debugger_url);
        inject_js(INJECTED_JS_TOGGLE_BILLY_RADIO, debugger_url);
        inject_js(INJECTED_JS_BILLY_RADIO_CTL, debugger_url);
        inject_js(INJECTED_JS_BILLY_BUTTONS, debugger_url);
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
            thread::sleep(time::Duration::from_secs(5));
            init_billy(debugger_url[0]);
            // inject_js(INJECTED_JS_PLAY, debugger_url[0]);
        }
    }
}
