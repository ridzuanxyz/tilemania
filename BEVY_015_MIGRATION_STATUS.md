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
- [ ] Install runtime dependencies (libxkbcommon-x11)
- [ ] Setup X server or native Windows build
- [ ] Verify runtime behavior matches expected
- [ ] Test all 5 game stages

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

## 📊 Migration Statistics

- **Total Errors Fixed**: 240+ → 0
- **Files Modified**: 50+
- **Commits**: 7
- **API Changes Applied**: 10+ different Bevy 0.15 patterns
- **Time**: Multiple sessions
- **Success Rate**: 100% ✅

---

## 📚 References

- [Bevy 0.15 Migration Guide](https://bevyengine.org/learn/migration-guides/0-14-to-0-15/)
- [Bevy 0.15 Required Components](https://bevyengine.org/news/bevy-0-15/#required-components)
- [Bevy 0.15 Text Rendering Changes](https://bevyengine.org/news/bevy-0-15/#text-rendering)
- [X11 Forwarding with WSL2](https://learn.microsoft.com/en-us/windows/wsl/tutorials/gui-apps)
- [VcXsrv Windows X Server](https://sourceforge.net/projects/vcxsrv/)
- [xrdp Documentation](https://github.com/neutrinolabs/xrdp)
