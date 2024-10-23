# CANlib

** Do not expect quality code out of this repo. I am not very good at rust**
a port of the Kvaser CANlib to rust. Based on the work done in [this repo](https://git.grepit.se/encrypted-can/lib). 

## Requirements
- Kvaser CANlib SDK: [website](https://kvaser.com/download/)
- Kvaser Drivers: [website](https://kvaser.com/download/)

the path to the canlib32 library must be included in your $PATH environment variable. On Windows the default location is: 
```C:\Program Files\Kvaser\Drivers\```

## Roadmap
This repo will be updated to support CAN features as the need arises. My current plan is to build out support for CANOpen, which will be tracked in the [CANOpen](https://github.com/parkermathis/CANOpen) repo.

Not all included functions may work as intended. 
