Please report any bugs and errors under "Issues" tab.

Usage is pretty simple:

sudo zenbook-cli  --[ARG] [OPTION]

ARGS:
- battery
- fan

Options:

Battery:
- max-blife: Charge limit to 60%
- optimal: Charge limit to 80%
- full: No limit.
- one-time: Charge to 100% only once.

Fan:
- full: Full fan speed.
- auto: Auto fan speed.

Tested on ASUS Zenbook 14 UM3406HA. Battery will limit probably work on any ASUS or even non-ASUS laptops but i'm not sure about fan control.