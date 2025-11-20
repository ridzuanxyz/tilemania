# Bevy 0.15 Migration Status

## ✅ MIGRATION COMPLETE

**Status**: All compilation errors resolved (0 errors)
**Date Completed**: 2025-11-19
**Total Errors Fixed**: 240+ → 0

---

## ✅ Completed Changes

### 1. Removed Incorrect Imports (22 files)
**Issue**: Attempted to import `bevy::text::TextStyle` and `bevy::ui::Style` which don't exist in Bevy 0.15.

**Solution**: Removed all incorrect import statements. The needed types (`Text`, `Text2d`, `TextFont`, `TextColor`, `Node`) are available in `bevy::prelude::*`.

**Files affected**: All stage files (1-5) in systems.rs, visuals.rs, ui.rs, pause.rs, powerups.rs, board.rs

### 2. Migrated Text2dBundle to Required Components (6 instances)
**Issue**: `Text2dBundle` is deprecated in Bevy 0.15. The old `TextStyle` struct for 2D text no longer exists.

**Old API**:
```rust
Text2dBundle {
    text: Text::from_section(
        "text",
        TextStyle {
            font: asset_server.load("font.ttf"),
            font_size: 48.0,
            color: Color::WHITE,
        },
    ),
    transform: Transform::from_xyz(x, y, z),
    ..default()
}
```

**New API** (Bevy 0.15):
```rust
(
    Text2d::new("text"),
    TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 48.0,
        ..default()
    },
    TextColor(Color::WHITE),
    Transform::from_xyz(x, y, z),
)
```

**Files migrated**:
- src/stage1/systems.rs
- src/stage1/powerups.rs
- src/stage1/visuals.rs
- src/stage2/systems.rs
- src/stage2/visuals.rs
- src/stage3/board.rs

### 3. Migrated All TextBundle to Required Components (57 instances)
**Issue**: `TextBundle` is deprecated in Bevy 0.15.

**Old API**:
```rust
TextBundle::from_section(
    "text",
    TextStyle {
        font: asset_server.load("font.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    },
)
```

**New API** (Bevy 0.15):
```rust
(
    Text::new("text"),
    TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 32.0,
        ..default()
    },
    TextColor(Color::WHITE),
    Node { ...optional styling... },
)
```

**Files fully migrated**:
- src/stage1/ui.rs, pause.rs
- src/stage2/ui.rs, pause.rs
- src/stage3/ui.rs, pause.rs
- src/stage4/ui.rs, pause.rs
- src/stage5/ui.rs, pause.rs

### 4. Text Query Updates (All stages)
**Issue**: Systems accessing `text.sections[0]` which no longer exists in Bevy 0.15.

**Old API**:
```rust
text.sections[0].value = "new text";
text.sections[0].style.color = Color::RED;
```

**New API** (Bevy 0.15):
```rust
// Direct text update
**text = "new text";

// Color update (query TextColor component)
text_color.0 = Color::RED;
```

**Files updated**:
- src/stage1/systems.rs, ui.rs, visuals.rs
- src/stage2/systems.rs, ui.rs, visuals.rs
- src/stage3/systems.rs, ui.rs, visuals.rs
- src/stage4/ui.rs, visuals.rs

### 5. Bundle Field Renames (44 instances)
**Issue**: `NodeBundle` and `ButtonBundle` renamed `style` field to `node` in Bevy 0.15.

**Change**: `style: Node { ... }` → `node: Node { ... }`

**Files updated**: All stage1-5 UI and pause files

### 6. ZIndex API Change (4 instances)
**Issue**: `ZIndex::Global` removed in Bevy 0.15.

**Change**: `ZIndex::Global(100)` → `ZIndex(100)`

**Files updated**:
- src/stage2/pause.rs
- src/stage3/pause.rs
- src/stage4/pause.rs
- src/stage5/pause.rs

### 7. Other API Updates
- **delta_seconds() → delta_secs()**: Fixed globally
- **viewport_to_world_2d**: Changed from `Option<Vec2>` to `Result<Vec2, Error>`
- **Handle<Font> type annotations**: Added explicit types where needed
- **calculate_word_score → calculate_score**: Updated method name and signature
- **Sprite mutation**: Fixed query parameters to use `mut` where needed
- **Borrow checker fixes**: Resolved conflicts by pre-collecting data in 4 files
- **System tuple limits**: Split large tuples to meet trait bounds
- **AudioEvent Debug derive**: Added missing Debug trait

---

## 🔧 System Dependencies

### Build-Time Dependencies
**Required for compilation** (Ubuntu/Debian):
```bash
sudo apt-get update
sudo apt-get install libasound2-dev libudev-dev pkg-config
```

**Required for compilation** (Fedora/RHEL):
```bash
sudo dnf install alsa-lib-devel systemd-devel pkg-config
```

**Required for compilation** (Arch):
```bash
sudo pacman -S alsa-lib systemd pkg-config
```

### Runtime Dependencies
**Required for running GUI** (Ubuntu/Debian):
```bash
sudo apt-get install libxkbcommon-x11-0 libxkbcommon-x11-dev
```

**Error if missing**:
```
Library libxkbcommon-x11.so could not be loaded.
```

---

## 🖥️ Running GUI Applications on Linux

### Overview
Bevy games require a display server to render graphics. When running on Linux (especially WSL2), you need to configure display forwarding.

### Option 1: X11 Forwarding (Recommended for WSL2)

X11 is the traditional Linux windowing system. It uses a client-server architecture where:
- **X Client**: Your Bevy game (running on Linux/WSL2)
- **X Server**: Receives graphics commands and displays windows (running on Windows host)

#### How it works:
1. X Server runs on Windows and listens for connections
2. Linux application connects to X Server via DISPLAY variable
3. Graphics commands sent over network/IPC to Windows
4. Windows displays the actual window

#### Setup for WSL2:

**Step 1: Install X Server on Windows**

Choose one:
- **VcXsrv** (Free, open-source): https://sourceforge.net/projects/vcxsrv/
- **Xming** (Free): https://sourceforge.net/projects/xming/
- **X410** (Paid, $10): Microsoft Store
- **MobaXterm** (Free with built-in X server): https://mobaxterm.mobatek.net/

**Step 2: Start X Server on Windows**

For VcXsrv:
1. Run XLaunch
2. Select "Multiple windows"
3. Select "Start no client"
4. **Important**: Check "Disable access control" (or configure firewall rules)
5. Finish and keep running

**Step 3: Configure DISPLAY in WSL2**

Add to `~/.bashrc` or `~/.zshrc`:
```bash
# For WSL2 - Get Windows host IP dynamically
export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}'):0

# Alternative: If using WSLg (Windows 11)
export DISPLAY=:0
```

Apply changes:
```bash
source ~/.bashrc
```

**Step 4: Test X11 forwarding**
```bash
# Install test app
sudo apt-get install x11-apps

# Test with simple GUI
xeyes
```

If `xeyes` opens, X11 forwarding works!

**Step 5: Run your Bevy game**
```bash
cargo run
```

#### Troubleshooting X11:

**"Cannot open display"**:
- Verify X Server is running on Windows
- Check DISPLAY variable: `echo $DISPLAY`
- Test with `xeyes`
- Check Windows Firewall (allow VcXsrv)

**Performance issues**:
- X11 forwarding can be slow for complex graphics
- Consider using native Windows build instead
- Or use Wayland with weston (more complex)

### Option 2: xrdp (Remote Desktop Protocol)

**What is xrdp?**
xrdp is a Remote Desktop Protocol (RDP) server for Linux. It allows you to connect to a Linux desktop using Microsoft Remote Desktop Client.

**Architecture**:
- Full Linux desktop environment runs on Linux server
- RDP server (xrdp) provides remote access
- Windows client connects via Remote Desktop
- Entire desktop session rendered on Linux, streamed to Windows

**Use Case**: Best for:
- Remote Linux servers (not WSL2)
- Running full desktop environment remotely
- Multiple users accessing same server
- Traditional remote desktop workflow

**NOT recommended for**:
- WSL2 (X11 forwarding is simpler)
- Single application (overhead of full desktop)
- Low-latency gaming (RDP adds latency)

#### Setup xrdp (for remote Linux servers):

**On Linux server**:
```bash
# Install xrdp and desktop environment
sudo apt-get update
sudo apt-get install xrdp xfce4 xfce4-goodies

# Start xrdp service
sudo systemctl enable xrdp
sudo systemctl start xrdp

# Allow RDP through firewall
sudo ufw allow 3389/tcp

# Create user if needed
sudo adduser gameuser
```

**On Windows client**:
1. Open Remote Desktop Connection (mstsc.exe)
2. Enter Linux server IP address
3. Login with Linux credentials
4. Desktop appears - now run your Bevy game from terminal

**Pros**:
- Full desktop environment
- Persistent sessions
- Multi-user support
- Works over network

**Cons**:
- More resource intensive
- Higher latency than X11
- Requires full desktop installation
- Overkill for single application

### Option 3: Native Windows Build

**Best for performance and simplicity**:

Build directly on Windows instead of WSL2:

**Prerequisites**:
1. Install Rust: https://rustup.rs/
2. Install Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/
   - Select "Desktop development with C++"

**Build and run**:
```cmd
cargo run
```

No X server or display forwarding needed!

### Comparison: X11 vs xrdp vs Native

| Method | Best For | Pros | Cons |
|--------|----------|------|------|
| **X11 Forwarding** | WSL2, single apps | Simple, lightweight, good performance | Requires X Server on Windows |
| **xrdp** | Remote servers, full desktop | Persistent sessions, multi-user | High overhead, latency |
| **Native Windows** | Local development | Best performance, no setup | Windows-only environment |

### Recommendation for WSL2 + Bevy:

1. **Development**: Use native Windows build (best performance)
2. **Testing Linux**: Use X11 forwarding with VcXsrv
3. **Remote server**: Use xrdp only if full desktop needed

For your TileMania game on WSL2, I recommend:
- **Quick test**: Set up X11 forwarding (30 seconds)
- **Serious development**: Build on Windows natively

### WSLg (Windows 11)

**Note**: Windows 11 includes WSLg (Windows Subsystem for Linux GUI), which automatically handles X server setup.

If on Windows 11:
```bash
# Just set DISPLAY
export DISPLAY=:0

# Run directly
cargo run
```

No additional X server installation needed!

---

## 🔧 Troubleshooting: Running the Game on Linux/xrdp

### Common Issues and Solutions

This section documents real-world issues encountered when running TileMania on WSL2 with xrdp, and their solutions. These apply to Chromebook deployment and remote Linux servers.

---

### Issue 1: "Failed to build event loop: XOpenDisplayFailed" ❌

**Error Message:**
```
thread 'main' panicked at bevy_winit-0.15.3/src/lib.rs:142:14:
Failed to build event loop: Os(OsError {
    error: XNotSupported(XOpenDisplayFailed)
})
```

**Cause:** The `DISPLAY` environment variable is not set correctly, or the game is running in the wrong terminal.

**Solutions:**

#### Solution A: DISPLAY Variable Not Set

The xrdp session uses display `:10` but the terminal doesn't inherit this automatically.

**Check current DISPLAY:**
```bash
echo $DISPLAY
```

**Common mistakes:**
- ❌ `DISPLAY=10` (missing colon)
- ❌ `DISPLAY=` (empty)
- ❌ `DISPLAY=:0` (wrong display number)
- ✅ `DISPLAY=:10` (correct for xrdp)

**Fix - Temporary:**
```bash
export DISPLAY=:10
echo $DISPLAY  # Should show ":10"
cargo run
```

**Fix - Permanent:**
```bash
echo 'export DISPLAY=:10' >> ~/.bashrc
source ~/.bashrc
cargo run
```

**Critical Detail:** The colon `:` is **mandatory**.
- `:10` means "local X display number 10"
- `10` is invalid and will cause XOpenDisplayFailed

#### Solution B: Running from Wrong Terminal

**❌ Wrong:** Running from regular WSL2 terminal (outside RDP)
- Regular WSL2 terminal has no access to xrdp's X server
- Will always fail with XOpenDisplayFailed

**✅ Correct:** Running from terminal inside RDP desktop
1. Connect to xrdp: Remote Desktop → `localhost:3389`
2. Open Terminal application inside the RDP desktop
3. Set DISPLAY: `export DISPLAY=:10`
4. Run game: `cargo run`

**How to verify you're in the right place:**
```bash
# Run this inside your terminal
echo $DISPLAY        # Should show :10 (after export)
ls /tmp/.X11-unix/   # Should show X10
ps aux | grep Xorg   # Should show Xorg process on :10
```

---

### Issue 2: Missing Fonts - Blank Buttons 🔲

**Symptom:** Game runs, but buttons have no text. Exit dialogs show two unlabeled boxes.

**Error in logs:**
```
ERROR bevy_asset::server: Path not found: /home/ridzu/tilemania/assets/fonts/FiraSans-Bold.ttf
ERROR bevy_asset::server: Path not found: /home/ridzu/tilemania/assets/fonts/FiraSans-Medium.ttf
```

**Cause:** Font files not present in `assets/fonts/` directory.

**Fix:**
```bash
# Create fonts directory
mkdir -p ~/tilemania/assets/fonts
cd ~/tilemania/assets/fonts

# Download FiraSans fonts (Mozilla's open-source font)
wget https://github.com/mozilla/Fira/raw/master/ttf/FiraSans-Bold.ttf
wget https://github.com/mozilla/Fira/raw/master/ttf/FiraSans-Medium.ttf
wget https://github.com/mozilla/Fira/raw/master/ttf/FiraSans-Regular.ttf

# Verify fonts exist
ls -la ~/tilemania/assets/fonts/

# Run game - buttons should now have labels
cargo run
```

**After fix:** All menu items, buttons, and dialogs will display text properly.

---

### Issue 3: Audio Warnings (Expected, Not Critical) 🔇

**Warnings in logs:**
```
ALSA lib pcm.c:2664:(snd_pcm_open_noupdate) Unknown PCM default
Cannot connect to server socket err = No such file or directory
jack server is not running or cannot be started
ALSA lib pulse.c:242:(pulse_connect) PulseAudio: Unable to connect
WARN bevy_audio::audio_output: No audio device found.
```

**Cause:** xrdp sessions typically don't have audio forwarding configured by default. WSL2 has no sound card access.

**Impact:** ⚠️ **Non-critical** - Game runs perfectly without audio. These are warnings, not errors.

**Solutions:**

**For Development/Testing:**
- Ignore these warnings - game functionality is unaffected
- Audio is not required for testing gameplay logic

**For Chromebook Deployment:**
- Chromebook has native audio hardware - these warnings won't appear
- Audio will work automatically on actual Linux hardware

**For xrdp Audio (Optional):**
If you need audio in xrdp sessions:
```bash
# Install PulseAudio for xrdp
sudo apt-get install pulseaudio pulseaudio-module-xrdp
sudo systemctl restart xrdp

# Configure PulseAudio
pulseaudio --start
```

Note: Audio through RDP adds latency and is generally not recommended for game development.

---

### Issue 4: Software Rendering Warning (Performance) 🐌

**Warning:**
```
WARN bevy_render::renderer: The selected adapter is using a driver that only supports
software rendering. This is likely to be very slow.
AdapterInfo { name: "llvmpipe (LLVM 15.0.7, 256 bits)", backend: Vulkan }
```

**Cause:** WSL2 with xrdp uses software rendering (CPU-based graphics) instead of GPU acceleration.

**Impact:**
- ⚠️ Game will run slower than on native hardware
- ✅ Acceptable for development and testing
- ❌ Not representative of Chromebook performance

**Why it happens:**
- xrdp doesn't support GPU passthrough by default
- WSL2's graphics virtualization is limited
- llvmpipe = Mesa software rasterizer (CPU rendering)

**Solutions:**

**For Development:**
- Accept slower performance - functionality testing is still valid
- Gameplay logic works regardless of rendering speed

**For Performance Testing:**
- Test on actual Chromebook hardware with real GPU
- Chromebook will use hardware acceleration (much faster)
- Or build natively on Windows for local testing

**WSL2 GPU Acceleration (Advanced):**
Windows 11 with WSLg supports GPU acceleration, but requires:
- Windows 11 (not Windows 10)
- Updated GPU drivers
- WSLg enabled (built into Windows 11)

---

### Issue 5: XRandR Display Size Warning ⚠️

**Warning:**
```
WARN winit::platform_impl::linux::x11::util::randr:
XRandR reported that the display's 0mm in size, which is certifiably insane
```

**Cause:** xrdp doesn't report physical display dimensions correctly.

**Impact:** None - Bevy uses logical pixels, not physical dimensions.

**Action:** Ignore this warning - it's cosmetic and doesn't affect functionality.

---

## ✅ Successful Run - What to Expect

When everything is working correctly, you'll see:

```bash
$ export DISPLAY=:10
$ cargo run

     Running `target/debug/tilemania`
INFO bevy_diagnostic: SystemInfo { os: "Linux 22.04 Ubuntu", ... }
INFO bevy_winit::system: Creating new window "TileMania - Word Tile Strategy Game"
INFO tilemania::plugins::state: 📺 Entering Splash screen
INFO tilemania::plugins::assets: 📦 Initializing asset loading system
INFO tilemania::plugins::core: 🎮 TileMania Core initialized
INFO tilemania::plugins::core: 📚 Lexicon: CSW24 (280,886 words)
INFO tilemania::plugins::assets: ✅ Asset loading complete! Progress: 100%
INFO tilemania::plugins::state: 📋 Entering Main Menu
```

**Expected behavior:**
- ✅ Game window opens in xrdp desktop
- ✅ Splash screen appears briefly
- ✅ Main menu loads with visible text
- ✅ All menu items clickable and working
- ✅ Can navigate: Menu → Settings → Game Board → Results
- ✅ Exit dialog has labeled "Yes" and "No" buttons

**Sprint 1 Status:**
- ✅ Settings system fully implemented (Sprint 1, Week 3)
- ⚠️ Gameplay shows "Coming in Sprint 2-4"
- ✅ UI navigation fully functional

---

## 📋 Quick Reference Commands

### Setup (One-time)

```bash
# Install xrdp and desktop
sudo apt-get update
sudo apt-get install -y xfce4 xfce4-goodies xrdp

# Configure xrdp
sudo systemctl enable xrdp
sudo systemctl start xrdp

# Download fonts
mkdir -p ~/tilemania/assets/fonts
cd ~/tilemania/assets/fonts
wget https://github.com/mozilla/Fira/raw/master/ttf/FiraSans-Bold.ttf
wget https://github.com/mozilla/Fira/raw/master/ttf/FiraSans-Medium.ttf

# Make DISPLAY permanent
echo 'export DISPLAY=:10' >> ~/.bashrc
```

### Daily Usage

```bash
# 1. Connect to xrdp from Windows
#    Remote Desktop Connection → localhost:3389

# 2. Open Terminal in xrdp desktop

# 3. Run the game
cd ~/tilemania
cargo run

# Or, if DISPLAY not set:
export DISPLAY=:10
cargo run
```

### Debugging

```bash
# Check DISPLAY variable
echo $DISPLAY              # Should show :10

# Check X server
ls /tmp/.X11-unix/         # Should show X10
ps aux | grep Xorg         # Should show Xorg :10

# Test X11 connection
sudo apt-get install x11-apps
xeyes                      # Should open window

# Check fonts
ls -la ~/tilemania/assets/fonts/

# Run with full backtrace
RUST_BACKTRACE=full cargo run
```

---

## 🎯 Deployment Notes

### For Chromebook Deployment:

When deploying to Chromebook Linux (Crostini):

**Different from WSL2/xrdp:**
- ✅ DISPLAY will likely be `:0` (not `:10`)
- ✅ GPU acceleration available (no software rendering warning)
- ✅ Audio hardware present (no ALSA warnings)
- ✅ No xrdp needed (native Linux environment)

**Same as WSL2/xrdp:**
- ✅ Fonts must be in `assets/fonts/`
- ✅ Compile with `cargo build --release` for production
- ✅ All Bevy 0.15 migrations apply

**Quick setup on Chromebook:**
```bash
# Install dependencies
sudo apt-get install libasound2-dev libudev-dev pkg-config

# Clone and build
git clone https://github.com/yourusername/tilemania
cd tilemania
cargo build --release

# Run
./target/release/tilemania
```

### For Remote Linux Server:

**Differences:**
- DISPLAY number varies (check with `echo $DISPLAY`)
- May use `:0`, `:10`, or other number depending on session
- Always verify: `ls /tmp/.X11-unix/`

**Best practice:**
```bash
# Auto-detect display
export DISPLAY=$(ls /tmp/.X11-unix/ | sed 's/X/:/g' | head -1)
cargo run
```

---

## 📋 Migration Checklist

- [x] Remove incorrect bevy::text::TextStyle imports
- [x] Remove incorrect bevy::ui::Style imports
- [x] Convert all Text2dBundle → Text2d + TextFont + TextColor (6/6)
- [x] Convert all TextBundle → Text + TextFont + TextColor + Node (57/57)
- [x] Update text query systems for new API (all stages)
- [x] Update NodeBundle/ButtonBundle style → node fields (44 instances)
- [x] Fix ZIndex::Global → ZIndex (4 instances)
- [x] Fix viewport_to_world_2d API (2 instances)
- [x] Fix borrow checker errors (4 instances)
- [x] Fix system tuple trait bounds (1 instance)
- [x] Test compilation - **0 ERRORS** ✅
- [x] Install runtime dependencies (libxkbcommon-x11)
- [x] Setup xrdp desktop environment
- [x] Configure DISPLAY variable (:10)
- [x] Download FiraSans fonts
- [x] Verify runtime behavior - **GAME RUNS** ✅
- [x] Test UI navigation (Menu → Settings → GameBoard → Results) ✅
- [x] Implement settings system (Sprint 1, Week 3) ✅
- [x] Settings persistence and UI complete ✅
- [ ] Test all 5 game stages (Sprint 2-4 implementation pending)

---

## 🎉 Sprint 1, Week 3: Settings System Implementation

**Status**: ✅ Complete (2025-11-20)

Following the successful Bevy 0.15 migration, Sprint 1 Week 3 delivered a fully functional settings system:

### Features Implemented

**Settings Resource** (`src/plugins/settings.rs`):
- GameSettings resource with persistence
- Audio preferences (music/SFX toggle + volume)
- Gameplay defaults (dictionary, timer, difficulty)
- TOML-based storage with platform-specific paths
- Auto-load on game startup

**Interactive UI** (`src/ui/settings.rs`):
- 7 configurable settings (music, SFX, volume, dictionary, timer, difficulty)
- Real-time updates with button interactions
- Volume controls with +/- buttons
- Cycle buttons for discrete options
- Save button for persistence
- ESC key for quick exit

**Technical Details**:
- 775 lines of new code (2 files)
- Uses Bevy 0.15 patterns (Text::new, TextFont, TextColor, Node)
- Clean separation: data layer (persistence) + UI layer (interaction)
- Settings accessible via `Res<GameSettings>` in any system

### Storage Location

Settings persist in TOML format:
- **Linux**: `~/.config/tilemania/settings.toml`
- **Windows**: `%APPDATA%\TileMania\settings.toml`

### Integration Ready

The settings system is ready for integration:
```rust
// Example: Audio system integration
fn play_music(settings: Res<GameSettings>, audio: Res<Audio>) {
    if settings.audio.music_enabled {
        audio.set_volume(settings.audio.music_volume);
    }
}
```

### Documentation

Complete documentation available:
- **[SETTINGS_SYSTEM.md](SETTINGS_SYSTEM.md)** - User guide + API reference
- **[SPRINT_1_WEEK_3_COMPLETION.md](SPRINT_1_WEEK_3_COMPLETION.md)** - Implementation summary

---

## 🎯 Next Steps

1. ✅ **Migration Complete** - Code compiles with 0 errors
2. **Install runtime dependencies**: `sudo apt-get install libxkbcommon-x11-0`
3. **Setup display**:
   - Option A: Configure X11 forwarding (VcXsrv + DISPLAY variable)
   - Option B: Build and run natively on Windows
   - Option C: Use WSLg on Windows 11 (automatic)
4. **Test the game**: `cargo run`
5. **Verify all stages work correctly**
6. **Performance testing** with the new required components pattern

---

## 📊 Migration & Sprint 1 Statistics

**Bevy 0.15 Migration:**
- **Total Errors Fixed**: 240+ → 0
- **Files Modified**: 50+
- **Commits**: 6 migration commits
- **API Changes Applied**: 10+ different Bevy 0.15 patterns
- **Time**: Multiple sessions
- **Success Rate**: 100% ✅

**Sprint 1, Week 3 (Settings System):**
- **Lines Added**: 775
- **Files Created**: 1 source + 2 docs
- **Files Modified**: 5
- **Commits**: 1
- **Build Time**: 1m 22s
- **Errors**: 0

---

## 📚 References

- [Bevy 0.15 Migration Guide](https://bevyengine.org/learn/migration-guides/0-14-to-0-15/)
- [Bevy 0.15 Required Components](https://bevyengine.org/news/bevy-0-15/#required-components)
- [Bevy 0.15 Text Rendering Changes](https://bevyengine.org/news/bevy-0-15/#text-rendering)
- [X11 Forwarding with WSL2](https://learn.microsoft.com/en-us/windows/wsl/tutorials/gui-apps)
- [VcXsrv Windows X Server](https://sourceforge.net/projects/vcxsrv/)
- [xrdp Documentation](https://github.com/neutrinolabs/xrdp)
