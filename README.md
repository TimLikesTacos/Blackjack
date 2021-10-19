

## Dependencies
The FLTK library used to create the GUI requires Rust, CMake, Git, and a C++11 compiler.  
Instructions from [FLTK's repository](https://github.com/fltk-rs/fltk-rs#dependencies) for installing these depedencies are copied here:  

For Debian-based GUI distributions, that means running:
```
sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libpng-dev libgl1-mesa-dev libglu1-mesa-dev
```
For RHEL-based GUI distributions, that means running:
```
sudo yum groupinstall "X Software Development" && yum install pango-devel libXinerama-devel libpng-devel
```
For Arch-based GUI distributions, that means running:
```
sudo pacman -S libx11 libxext libxft libxinerama libxcursor libxrender libxfixes libpng pango cairo libgl mesa --needed
```
For Alpine linux:
```
apk add pango-dev fontconfig-dev libxinerama-dev libxfixes-dev libxcursor-dev libpng-dev mesa-gl
```
For NixOS (Linux distribution) this nix-shell environment can be used:
```
nix-shell --packages rustc cmake git gcc xorg.libXext xorg.libXft xorg.libXinerama xorg.libXcursor x
```
