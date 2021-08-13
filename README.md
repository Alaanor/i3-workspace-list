# i3-workspace-list

smol program to simply list which workspace are currently being used by i3, written in rust just because i can.

## Motivation

I wanted to do a [eww](https://github.com/elkowar/eww) widget and I needed the info about the current workspace being
used and found no way to get the info through cli.

i wrote this in a single night pls no hit

## Usage

```bash
i3-workspace-list 0.1

USAGE:
    i3-workspace-list [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --name <name>          
        --num <num>            
        --screen <screen>      
        --visible <visible> 
```

### Get all workspaces

```bash
> i3-workspace-list
Workspace { id: 94120323101968, num: 1, name: "1", visible: true, focused: false, urgent: false, rect: Rect { x: 0, y: 360, width: 1920, height: 1080 }, output: "DP-0" }
Workspace { id: 94120323098848, num: 3, name: "3", visible: true, focused: false, urgent: false, rect: Rect { x: 4480, y: 360, width: 1920, height: 1080 }, output: "HDMI-0" }
Workspace { id: 94120323139808, num: 6, name: "6", visible: false, focused: false, urgent: false, rect: Rect { x: 4480, y: 360, width: 1920, height: 1080 }, output: "HDMI-0" }
Workspace { id: 94120323095728, num: 2, name: "2", visible: true, focused: true, urgent: false, rect: Rect { x: 1920, y: 24, width: 2560, height: 1416 }, output: "DP-2" }
```

### Is workspace num 2 used ?

```bash
> i3-workspace-list --num 2 | wc -l
1
```