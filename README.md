# MinecraftStarter
This is a small tool that makes the Twitch Launcher go to the minecraft tab on startup.

# Tutorial
1. Download & Extract the MinecraftStarter.zip file from the release tab (Or compile the exe yourself if you want to)
2. Copy the MinecraftStarter.exe file into your Twitch Binary Path (Default is C:\Users\USERNAME\AppData\Roaming\Twitch\Bin)
3. "Run" the Enable_MinecraftStarter.reg file.
4. Done

To undo the whole thing just run the Disable_MinecraftStarter.reg file and remove the MinecraftStarter.exe file from your twitch folder.

It works fine for me, no guarantee it does for you :).

# How it works
The "home page" of the launcher is a simple command line argument for the TwitchUI. The registry file causes the MinecraftStarter.exe to run instead of the actual twitch ui. 
The MinecraftStarter.exe then changes the home page in the command line arguments and runs the actual TwitchUI. (It also copies the TwitchUI.exe because it wouldn't run with DEBUG_PROCESS)
