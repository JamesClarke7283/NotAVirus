# ThisIsNoRickRoll

The purpose of this script is to simulate a malware for educational purposes.
Which:
1. Disables all input devices
2. Sends the user text conversation like so:
```rust
fn simulate_conversation() {
    let conversation = vec![
        ("Unknown", "Hi!"),
        ("You", "Who are you?"),
        ("Unknown", "A Unknown Guy."),
        ("You", "Hmm. Okay. What do u want?"),
        ("Unknown", "YOUR PC!!!"),
        ("You", "WAIT!!! TELL ME WHY I CANT USE MY KEYBOARD?!?"),
        ("Unknown", "We want your PC but dont worry we turn it on later."),
        ("Unknown", "Just give us ur Password and we turn it on"),
        ("You", "Ok but pls dont hack my PC."),
        ("Unknown", "Ok ur keyboard is on. Enjoy your freetime!"),
    ];

    for (speaker, message) in conversation {
        println!("{}: {}", speaker, message);
        thread::sleep(Duration::from_secs(1));
    }
}
```
3. Reenables all the input devices as normal.

The original bash script is as follows:
```bash
if [ "$EUID" -ne 0 ]
  then echo "Please run as root/sudo."
  exit
fi

device_ids=$(xinput list --id-only)
echo "Install xdotool..."
sudo apt install xdotool -y

for device_id in $device_ids; do
    xinput disable "$device_id"
done

echo "Unknown: Hi!" && sleep 1
echo "You: Who are you?" && sleep 1
echo "Unknow: A Unknown Guy." && sleep 1
echo "You: Hmm. Okay. What do u want?" && sleep 1
echo "Unknow: YOUR PC!!!" && sleep 1
echo "You: WAIT!!! TELL ME WHY I CANT USE MY KEYBOARD?!?" && sleep 1
echo "Unknow: We want your PC but dont worry we turn it on later." && sleep 1
echo "Unknow: Just give us ur Password and we turn it on" && sleep 1
echo "You: Ok but pls dont hack my PC." && sleep 1
echo "Unknow: Ok ur keyboard is on. Enjoy your freetime!"

for device_id in $device_ids; do
    xinput enable "$device_id"
done
```

## Improved Version
The goal of the rust script is to improve on the bash script so it runs on all major platforms.
### Platforms supported
- GNU/Linux X11
- GNU/Linux Wayland
- MacOSX: Darwin
- Windows

### Improvements
It will detect what linux distro is installed, and install the needed tooling to disable and enable the input.
It will also detect the os and decide to use the required tool for the job to do it.
It will spawn a terminal window first, which will have all the conversation inside it. but the program will be running in the background outside the terminal session.
Unlike the bash script, this program should be able to run under wayland as well.
